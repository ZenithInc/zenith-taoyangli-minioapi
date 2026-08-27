pub mod cache;
pub mod config;
pub mod contract;
pub mod object_store;
pub mod php_cache;
pub mod request_id;
pub mod token;
pub mod upload;
pub mod wechat;

use std::{net::IpAddr, sync::Arc};

use axum::{
    Json, Router,
    body::Bytes,
    extract::{ConnectInfo, DefaultBodyLimit, Extension, Multipart, State},
    http::{HeaderMap, StatusCode},
    middleware,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde_json::json;
use tokio::sync::Semaphore;
use tracing::{error, info};

use crate::{
    cache::CacheStore,
    contract::{
        BAD_REQUEST_CODE, LegacyResponse, SERVER_ERROR_CODE, UPLOAD_ERROR_CODE, parse_params,
    },
    object_store::ObjectStore,
    request_id::RequestId,
    token::{TokenKind, TokenService},
    upload::{UploadError, prepare_stream},
};

#[derive(Clone)]
pub struct AppState {
    pub tokens: TokenService,
    pub cache: Arc<dyn CacheStore>,
    pub objects: Arc<dyn ObjectStore>,
    pub uploads: Arc<Semaphore>,
    pub temp_dir: std::path::PathBuf,
}

const INTERNAL_PROBE_HEADER: &str = "x-taoyangli-internal-probe";

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/internal/dependencyz", get(dependencyz))
        .route("/wxmini/accesstoken", post(mini_access_token))
        .route("//wxmini/accesstoken", post(mini_access_token))
        .route("/wx/accesstoken", post(official_access_token))
        .route("//wx/accesstoken", post(official_access_token))
        .route("/wx/jsTicket", post(js_ticket))
        .route("//wx/jsTicket", post(js_ticket))
        .route("/upload", post(upload))
        .route("//upload", post(upload))
        .fallback(unknown)
        .layer(DefaultBodyLimit::max(102 * 1024 * 1024))
        .layer(middleware::from_fn(request_id::middleware))
        .with_state(state)
}

async fn healthz(Extension(request_id): Extension<RequestId>, headers: HeaderMap) -> Response {
    if !internal_probe(&headers) {
        return unknown_body(request_id).into_response();
    }
    Json(json!({"status":"ok"})).into_response()
}

async fn readyz(Extension(request_id): Extension<RequestId>, headers: HeaderMap) -> Response {
    if !internal_probe(&headers) {
        return unknown_body(request_id).into_response();
    }
    Json(json!({"status":"ready"})).into_response()
}

async fn dependencyz(
    ConnectInfo(peer): ConnectInfo<std::net::SocketAddr>,
    Extension(request_id): Extension<RequestId>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Response {
    if !internal_probe(&headers) {
        return unknown_body(request_id).into_response();
    }
    let forwarded_for = match headers.get("x-forwarded-for") {
        None => None,
        Some(value) => match value.to_str() {
            Ok(value) => Some(value),
            Err(_) => {
                return (StatusCode::NOT_FOUND, Json(json!({"status":"not_found"})))
                    .into_response();
            }
        },
    };
    if !private_request(peer.ip(), forwarded_for) {
        return (StatusCode::NOT_FOUND, Json(json!({"status":"not_found"}))).into_response();
    }
    let (cache, object_store) = tokio::join!(state.cache.ping(), state.objects.health());
    let ok = cache.is_ok() && object_store.is_ok();
    let status = if ok {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };
    (
        status,
        Json(json!({
            "status": if ok { "ok" } else { "failed" },
            "dependencies": {
                "redis": if cache.is_ok() { "ok" } else { "failed" },
                "object_storage": if object_store.is_ok() { "ok" } else { "failed" }
            }
        })),
    )
        .into_response()
}

async fn mini_access_token(
    State(state): State<AppState>,
    Extension(request_id): Extension<RequestId>,
    headers: HeaderMap,
    body: Bytes,
) -> Json<LegacyResponse> {
    token_response(state, request_id, TokenKind::MiniAccessToken, headers, body).await
}

async fn official_access_token(
    State(state): State<AppState>,
    Extension(request_id): Extension<RequestId>,
    headers: HeaderMap,
    body: Bytes,
) -> Json<LegacyResponse> {
    token_response(
        state,
        request_id,
        TokenKind::OfficialAccessToken,
        headers,
        body,
    )
    .await
}

async fn js_ticket(
    State(state): State<AppState>,
    Extension(request_id): Extension<RequestId>,
    headers: HeaderMap,
    body: Bytes,
) -> Json<LegacyResponse> {
    token_response(state, request_id, TokenKind::JsApiTicket, headers, body).await
}

async fn token_response(
    state: AppState,
    request_id: RequestId,
    kind: TokenKind,
    headers: HeaderMap,
    body: Bytes,
) -> Json<LegacyResponse> {
    let params = match parse_params(&headers, &body) {
        Ok(params) => params,
        Err(_) => {
            return LegacyResponse::failure(request_id.0, BAD_REQUEST_CODE, "请求参数错误");
        }
    };
    match state.tokens.get(kind, &params).await {
        Ok(data) => LegacyResponse::success(request_id.0, "获取成功", data),
        Err(failure) => LegacyResponse::failure(request_id.0, failure.code, failure.message),
    }
}

async fn upload(
    State(state): State<AppState>,
    Extension(request_id): Extension<RequestId>,
    mut multipart: Multipart,
) -> Json<LegacyResponse> {
    let permit = match state.uploads.clone().try_acquire_owned() {
        Ok(permit) => permit,
        Err(_) => {
            return LegacyResponse::failure(
                request_id.0,
                UPLOAD_ERROR_CODE,
                "上传任务繁忙，请稍后重试",
            );
        }
    };

    while let Ok(Some(field)) = multipart.next_field().await {
        if field.name() != Some("upload_file") {
            continue;
        }
        let Some(filename) = field.file_name().map(ToOwned::to_owned) else {
            return LegacyResponse::failure(request_id.0, UPLOAD_ERROR_CODE, "上传失败，请重试");
        };
        let prepared = match prepare_stream(&filename, &state.temp_dir, field).await {
            Ok(prepared) => prepared,
            Err(UploadError::UnsupportedExtension) => {
                return LegacyResponse::failure(
                    request_id.0,
                    UPLOAD_ERROR_CODE,
                    "上传文件格式不正确",
                );
            }
            Err(UploadError::InvalidFilename) => {
                return LegacyResponse::failure(
                    request_id.0,
                    UPLOAD_ERROR_CODE,
                    "上传文件名不合法",
                );
            }
            Err(UploadError::TooLarge) => {
                return LegacyResponse::failure(
                    request_id.0,
                    UPLOAD_ERROR_CODE,
                    "上传文件超过100MB限制",
                );
            }
            Err(UploadError::Interrupted | UploadError::Storage) => {
                return LegacyResponse::failure(
                    request_id.0,
                    UPLOAD_ERROR_CODE,
                    "上传失败，请重试",
                );
            }
        };

        let object_url = match state
            .objects
            .put(&prepared.path, &prepared.object_name, prepared.content_type)
            .await
        {
            Ok(url) => url,
            Err(_) => {
                error!(request_id = %request_id.0, "object storage upload failed");
                return LegacyResponse::failure(request_id.0, UPLOAD_ERROR_CODE, "上传失败");
            }
        };
        info!(
            request_id = %request_id.0,
            size = prepared.size,
            "upload completed"
        );
        drop(permit);
        return LegacyResponse::success(
            request_id.0,
            "上传成功",
            json!({"url": object_url, "size": prepared.size}),
        );
    }

    LegacyResponse::failure(request_id.0, UPLOAD_ERROR_CODE, "上传失败，请重试")
}

async fn unknown(Extension(request_id): Extension<RequestId>) -> Json<LegacyResponse> {
    unknown_body(request_id)
}

fn unknown_body(request_id: RequestId) -> Json<LegacyResponse> {
    LegacyResponse::failure(request_id.0, SERVER_ERROR_CODE, "服务器异常")
}

fn internal_probe(headers: &HeaderMap) -> bool {
    headers
        .get(INTERNAL_PROBE_HEADER)
        .is_some_and(|value| value.as_bytes() == b"1")
}

pub fn private_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(ip) => ip.is_private() || ip.is_loopback() || ip.is_link_local(),
        IpAddr::V6(ip) => ip.is_loopback() || ip.is_unique_local() || ip.is_unicast_link_local(),
    }
}

pub fn private_request(peer: IpAddr, forwarded_for: Option<&str>) -> bool {
    if !private_ip(peer) {
        return false;
    }
    let Some(forwarded_for) = forwarded_for else {
        return true;
    };
    if forwarded_for.trim().is_empty() {
        return true;
    }
    forwarded_for.split(',').all(|address| {
        address
            .trim()
            .parse::<IpAddr>()
            .ok()
            .is_some_and(private_ip)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        cache::{CacheError, CacheStore},
        config::TokenMode,
        contract::SUCCESS_CODE,
        object_store::{ObjectStore, ObjectStoreError},
        wechat::{WechatApi, WechatError},
    };
    use async_trait::async_trait;
    use axum::{body::Body, http::Request};
    use http_body_util::BodyExt;
    use serde_json::Value;
    use std::{collections::HashMap, path::Path, sync::Mutex};
    use tower::ServiceExt;

    #[derive(Default)]
    struct FakeCache {
        values: Mutex<HashMap<String, Vec<u8>>>,
    }

    #[async_trait]
    impl CacheStore for FakeCache {
        async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, CacheError> {
            Ok(self.values.lock().unwrap().get(key).cloned())
        }
        async fn set_ex(&self, key: &str, value: &[u8], _: u64) -> Result<(), CacheError> {
            self.values.lock().unwrap().insert(key.into(), value.into());
            Ok(())
        }
        async fn try_lock(&self, _: &str, _: &str, _: u64) -> Result<bool, CacheError> {
            Ok(true)
        }
        async fn unlock(&self, _: &str, _: &str) -> Result<(), CacheError> {
            Ok(())
        }
        async fn ping(&self) -> Result<(), CacheError> {
            Ok(())
        }
    }

    struct FakeWechat;
    #[async_trait]
    impl WechatApi for FakeWechat {
        async fn get(&self, _: &str, _: &HashMap<&str, String>) -> Result<Value, WechatError> {
            panic!("shadow endpoint test must not call WeChat")
        }
    }

    struct FakeObjects;
    #[async_trait]
    impl ObjectStore for FakeObjects {
        async fn put(&self, _: &Path, name: &str, _: &str) -> Result<String, ObjectStoreError> {
            Ok(format!("https://objects.invalid/{name}"))
        }
        async fn health(&self) -> Result<(), ObjectStoreError> {
            Ok(())
        }
    }

    fn test_app() -> Router {
        let cache = Arc::new(FakeCache::default());
        let packed =
            crate::php_cache::encode(&json!({"access_token":"fixture","expires_in":7200})).unwrap();
        cache
            .values
            .lock()
            .unwrap()
            .insert("c:tools:wx123wx_access_token".into(), packed);
        let tokens = TokenService::new(
            cache.clone(),
            Arc::new(FakeWechat),
            TokenMode::ShadowReadonly,
            "tools",
        );
        app(AppState {
            tokens,
            cache,
            objects: Arc::new(FakeObjects),
            uploads: Arc::new(Semaphore::new(2)),
            temp_dir: std::env::temp_dir(),
        })
    }

    #[tokio::test]
    async fn token_endpoint_and_double_slash_are_contract_compatible() {
        for path in ["/wxmini/accesstoken", "//wxmini/accesstoken"] {
            let response = test_app()
                .oneshot(
                    Request::post(path)
                        .header("content-type", "application/x-www-form-urlencoded")
                        .header("x-request-id", "fixture-id")
                        .body(Body::from("app_id=wx123&secret=not-logged"))
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::OK);
            assert_eq!(response.headers()["x-request-id"], "fixture-id");
            let body: Value =
                serde_json::from_slice(&response.into_body().collect().await.unwrap().to_bytes())
                    .unwrap();
            assert_eq!(body["return_code"], SUCCESS_CODE);
            assert_eq!(body["data"]["access_token"], "fixture");
        }
    }

    #[tokio::test]
    async fn oversize_request_id_is_replaced() {
        let response = test_app()
            .oneshot(
                Request::post("/wxmini/accesstoken")
                    .header("content-type", "application/x-www-form-urlencoded")
                    .header("x-request-id", "a".repeat(65))
                    .body(Body::from("app_id=wx123"))
                    .unwrap(),
            )
            .await
            .unwrap();
        let id = response.headers()["x-request-id"].to_str().unwrap();
        assert_ne!(id, "a".repeat(65));
        assert!(crate::request_id::valid(id));
    }

    #[tokio::test]
    async fn health_routes_preserve_unknown_contract_without_internal_marker() {
        let public = test_app()
            .oneshot(
                Request::get("/healthz")
                    .header("x-request-id", "public-health-fixture")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(public.status(), StatusCode::OK);
        let body: Value =
            serde_json::from_slice(&public.into_body().collect().await.unwrap().to_bytes())
                .unwrap();
        assert_eq!(body["return_code"], SERVER_ERROR_CODE);
        assert_eq!(body["requestId"], "public-health-fixture");

        let internal = test_app()
            .oneshot(
                Request::get("/healthz")
                    .header(INTERNAL_PROBE_HEADER, "1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(internal.status(), StatusCode::OK);
        let body: Value =
            serde_json::from_slice(&internal.into_body().collect().await.unwrap().to_bytes())
                .unwrap();
        assert_eq!(body["status"], "ok");
    }

    #[tokio::test]
    async fn upload_succeeds_without_authentication_and_preserves_legacy_response_shape() {
        let boundary = "fixture-boundary";
        let body = format!(
            "--{boundary}\r\nContent-Disposition: form-data; name=\"upload_file\"; filename=\"fixture.png\"\r\nContent-Type: application/octet-stream\r\n\r\npng-bytes\r\n--{boundary}--\r\n"
        );
        let response = test_app()
            .oneshot(
                Request::post("//upload")
                    .header(
                        "content-type",
                        format!("multipart/form-data; boundary={boundary}"),
                    )
                    .header("qid", "upload-fixture")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body: Value =
            serde_json::from_slice(&response.into_body().collect().await.unwrap().to_bytes())
                .unwrap();
        assert_eq!(body["requestId"], "upload-fixture");
        assert_eq!(body["return_code"], SUCCESS_CODE);
        assert_eq!(body["msg"], "上传成功");
        assert_eq!(body["data"]["size"], 9);
        assert!(
            body["data"]["url"]
                .as_str()
                .unwrap()
                .ends_with("_fixture.png")
        );
    }

    #[test]
    fn dependency_endpoint_ip_filter_is_private_only() {
        assert!(private_ip("10.0.0.1".parse().unwrap()));
        assert!(private_ip("127.0.0.1".parse().unwrap()));
        assert!(!private_ip("8.8.8.8".parse().unwrap()));
        assert!(!private_ip("0.0.0.0".parse().unwrap()));
        assert!(!private_ip("203.0.113.10".parse().unwrap()));
        assert!(private_request(
            "10.42.0.1".parse().unwrap(),
            Some("192.168.1.20, 10.0.0.1")
        ));
        assert!(private_request("10.42.0.1".parse().unwrap(), None));
        assert!(!private_request(
            "10.42.0.1".parse().unwrap(),
            Some("8.8.8.8, 10.0.0.1")
        ));
        assert!(!private_request(
            "10.42.0.1".parse().unwrap(),
            Some("10.0.0.1, 8.8.8.8")
        ));
        assert!(!private_request(
            "8.8.8.8".parse().unwrap(),
            Some("192.168.1.20")
        ));
    }
}
