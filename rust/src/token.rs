use std::{collections::HashMap, sync::Arc, time::Duration};

use serde_json::Value;
use thiserror::Error;
use tokio::time::sleep;
use uuid::Uuid;

use crate::{cache::CacheStore, config::TokenMode, contract::BAD_REQUEST_CODE, wechat::WechatApi};

#[derive(Clone, Copy, Debug)]
pub enum TokenKind {
    MiniAccessToken,
    OfficialAccessToken,
    JsApiTicket,
}

#[derive(Debug, Error)]
#[error("token operation failed")]
pub struct TokenError {
    pub code: i64,
    pub message: String,
}

impl TokenError {
    fn internal(message: impl Into<String>) -> Self {
        Self {
            code: BAD_REQUEST_CODE,
            message: message.into(),
        }
    }
}

#[derive(Clone)]
pub struct TokenService {
    cache: Arc<dyn CacheStore>,
    wechat: Arc<dyn WechatApi>,
    mode: TokenMode,
    cache_prefix: String,
}

impl TokenService {
    pub fn new(
        cache: Arc<dyn CacheStore>,
        wechat: Arc<dyn WechatApi>,
        mode: TokenMode,
        app_name: &str,
    ) -> Self {
        Self {
            cache,
            wechat,
            mode,
            cache_prefix: format!("c:{app_name}:"),
        }
    }

    pub async fn get(
        &self,
        kind: TokenKind,
        params: &HashMap<String, String>,
    ) -> Result<Value, TokenError> {
        let app_id = params.get("app_id").cloned().unwrap_or_default();
        match kind {
            TokenKind::MiniAccessToken => {
                let key = self.cache_key(kind, &app_id);
                self.cached_or_refresh(&key, 7_000, "/cgi-bin/token", token_query(params))
                    .await
            }
            TokenKind::OfficialAccessToken => self.get_official(params, &app_id).await,
            TokenKind::JsApiTicket => {
                let access_token = self
                    .get_official(params, &app_id)
                    .await?
                    .get("access_token")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_owned();
                let key = self.cache_key(kind, &app_id);
                let query =
                    HashMap::from([("access_token", access_token), ("type", "jsapi".to_owned())]);
                self.cached_or_refresh(&key, 5_400, "/cgi-bin/ticket/getticket", query)
                    .await
            }
        }
    }

    async fn get_official(
        &self,
        params: &HashMap<String, String>,
        app_id: &str,
    ) -> Result<Value, TokenError> {
        let key = self.cache_key(TokenKind::OfficialAccessToken, app_id);
        self.cached_or_refresh(&key, 5_400, "/cgi-bin/token", token_query(params))
            .await
    }

    pub fn cache_key(&self, kind: TokenKind, app_id: &str) -> String {
        match kind {
            TokenKind::MiniAccessToken => {
                format!("{}{app_id}wx_access_token", self.cache_prefix)
            }
            TokenKind::OfficialAccessToken => format!(
                "{}wx_access_token_{:x}",
                self.cache_prefix,
                md5::compute(format!("getAccessToken{app_id}"))
            ),
            TokenKind::JsApiTicket => format!(
                "{}wx_get_ticket_{:x}",
                self.cache_prefix,
                md5::compute(format!("getJsApiTicket{app_id}"))
            ),
        }
    }

    async fn cached_or_refresh(
        &self,
        key: &str,
        ttl: u64,
        path: &str,
        query: HashMap<&str, String>,
    ) -> Result<Value, TokenError> {
        if let Some(value) = self.read_cache(key).await? {
            return Ok(value);
        }
        if self.mode == TokenMode::ShadowReadonly {
            return Err(TokenError::internal("缓存未命中"));
        }

        let lock_key = format!("{key}:refresh-lock");
        let owner = Uuid::new_v4().simple().to_string();
        if !self
            .cache
            .try_lock(&lock_key, &owner, 30)
            .await
            .map_err(|_| TokenError::internal("缓存服务异常"))?
        {
            for _ in 0..20 {
                sleep(Duration::from_millis(100)).await;
                if let Some(value) = self.read_cache(key).await? {
                    return Ok(value);
                }
            }
            return Err(TokenError::internal("令牌刷新中，请重试"));
        }

        let result = self.refresh(key, ttl, path, &query).await;
        let _ = self.cache.unlock(&lock_key, &owner).await;
        result
    }

    async fn read_cache(&self, key: &str) -> Result<Option<Value>, TokenError> {
        let packed = self
            .cache
            .get(key)
            .await
            .map_err(|_| TokenError::internal("缓存服务异常"))?;
        packed
            .map(|bytes| {
                crate::php_cache::decode(&bytes).map_err(|_| TokenError::internal("缓存格式不兼容"))
            })
            .transpose()
    }

    async fn refresh(
        &self,
        key: &str,
        ttl: u64,
        path: &str,
        query: &HashMap<&str, String>,
    ) -> Result<Value, TokenError> {
        let response = self
            .wechat
            .get(path, query)
            .await
            .map_err(|_| TokenError::internal("接口请求失败"))?;
        if let Some(code) = response.get("errcode").and_then(Value::as_i64)
            && code != 0
        {
            return Err(TokenError {
                code,
                message: response
                    .get("errmsg")
                    .and_then(Value::as_str)
                    .unwrap_or("微信接口请求失败")
                    .to_owned(),
            });
        }
        let packed = crate::php_cache::encode(&response)
            .map_err(|_| TokenError::internal("缓存格式不兼容"))?;
        self.cache
            .set_ex(key, &packed, ttl)
            .await
            .map_err(|_| TokenError::internal("缓存服务异常"))?;
        Ok(response)
    }
}

fn token_query(params: &HashMap<String, String>) -> HashMap<&'static str, String> {
    HashMap::from([
        ("appid", params.get("app_id").cloned().unwrap_or_default()),
        ("secret", params.get("secret").cloned().unwrap_or_default()),
        ("grant_type", "client_credential".to_owned()),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        cache::{CacheError, CacheStore},
        wechat::{WechatApi, WechatError},
    };
    use async_trait::async_trait;
    use std::{
        collections::HashMap as Map,
        sync::{
            Mutex,
            atomic::{AtomicU32, Ordering},
        },
        time::Duration,
    };

    #[derive(Default)]
    struct MemoryCache {
        values: Mutex<Map<String, Vec<u8>>>,
        locks: Mutex<Map<String, String>>,
    }

    #[async_trait]
    impl CacheStore for MemoryCache {
        async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, CacheError> {
            Ok(self.values.lock().unwrap().get(key).cloned())
        }

        async fn set_ex(&self, key: &str, value: &[u8], _: u64) -> Result<(), CacheError> {
            self.values
                .lock()
                .unwrap()
                .insert(key.to_owned(), value.to_vec());
            Ok(())
        }

        async fn try_lock(&self, key: &str, owner: &str, _: u64) -> Result<bool, CacheError> {
            let mut locks = self.locks.lock().unwrap();
            if locks.contains_key(key) {
                Ok(false)
            } else {
                locks.insert(key.to_owned(), owner.to_owned());
                Ok(true)
            }
        }

        async fn unlock(&self, key: &str, owner: &str) -> Result<(), CacheError> {
            let mut locks = self.locks.lock().unwrap();
            if locks.get(key).map(String::as_str) == Some(owner) {
                locks.remove(key);
            }
            Ok(())
        }

        async fn ping(&self) -> Result<(), CacheError> {
            Ok(())
        }
    }

    struct FakeWechat {
        calls: Mutex<u32>,
    }

    struct SlowWechat {
        calls: AtomicU32,
    }

    #[async_trait]
    impl WechatApi for SlowWechat {
        async fn get(&self, _: &str, _: &Map<&str, String>) -> Result<Value, WechatError> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            tokio::time::sleep(Duration::from_millis(50)).await;
            Ok(serde_json::json!({"access_token":"fixture-token","expires_in":7200}))
        }
    }

    #[async_trait]
    impl WechatApi for FakeWechat {
        async fn get(&self, _: &str, _: &Map<&str, String>) -> Result<Value, WechatError> {
            *self.calls.lock().unwrap() += 1;
            Ok(serde_json::json!({"access_token":"fixture-token","expires_in":7200}))
        }
    }

    fn service(mode: TokenMode) -> (TokenService, Arc<MemoryCache>, Arc<FakeWechat>) {
        let cache = Arc::new(MemoryCache::default());
        let wechat = Arc::new(FakeWechat {
            calls: Mutex::new(0),
        });
        (
            TokenService::new(cache.clone(), wechat.clone(), mode, "tools"),
            cache,
            wechat,
        )
    }

    #[tokio::test]
    async fn shadow_mode_never_calls_wechat_or_writes() {
        let (service, cache, wechat) = service(TokenMode::ShadowReadonly);
        let result = service
            .get(
                TokenKind::MiniAccessToken,
                &Map::from([("app_id".to_owned(), "wx123".to_owned())]),
            )
            .await;
        assert!(result.is_err());
        assert_eq!(*wechat.calls.lock().unwrap(), 0);
        assert!(cache.values.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn active_mode_writes_php_serialized_cache_once() {
        let (service, cache, wechat) = service(TokenMode::Active);
        let params = Map::from([
            ("app_id".to_owned(), "wx123".to_owned()),
            ("secret".to_owned(), "fixture-secret".to_owned()),
        ]);
        let first = service
            .get(TokenKind::MiniAccessToken, &params)
            .await
            .unwrap();
        let second = service
            .get(TokenKind::MiniAccessToken, &params)
            .await
            .unwrap();
        assert_eq!(first, second);
        assert_eq!(*wechat.calls.lock().unwrap(), 1);
        let packed = cache
            .values
            .lock()
            .unwrap()
            .get("c:tools:wx123wx_access_token")
            .cloned()
            .unwrap();
        let decoded = crate::php_cache::decode(&packed).unwrap();
        assert_eq!(decoded, first);
    }

    #[tokio::test]
    async fn concurrent_refresh_has_exactly_one_active_writer() {
        let cache = Arc::new(MemoryCache::default());
        let wechat = Arc::new(SlowWechat {
            calls: AtomicU32::new(0),
        });
        let service = TokenService::new(cache, wechat.clone(), TokenMode::Active, "tools");
        let params = Map::from([
            ("app_id".to_owned(), "wx-concurrent".to_owned()),
            ("secret".to_owned(), "fixture-secret".to_owned()),
        ]);

        let (first, second) = tokio::join!(
            service.get(TokenKind::MiniAccessToken, &params),
            service.get(TokenKind::MiniAccessToken, &params)
        );

        assert_eq!(first.unwrap(), second.unwrap());
        assert_eq!(wechat.calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn cache_keys_are_byte_compatible_with_php_algorithm() {
        let (service, _, _) = service(TokenMode::Active);
        assert_eq!(
            service.cache_key(TokenKind::MiniAccessToken, "wx123"),
            "c:tools:wx123wx_access_token"
        );
        assert_eq!(
            service.cache_key(TokenKind::OfficialAccessToken, "wx123"),
            format!(
                "c:tools:wx_access_token_{:x}",
                md5::compute("getAccessTokenwx123")
            )
        );
        assert_eq!(
            service.cache_key(TokenKind::JsApiTicket, "wx123"),
            format!(
                "c:tools:wx_get_ticket_{:x}",
                md5::compute("getJsApiTicketwx123")
            )
        );
    }
}
