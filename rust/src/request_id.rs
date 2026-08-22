use axum::{
    extract::Request,
    http::{HeaderName, HeaderValue},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct RequestId(pub String);

pub async fn middleware(mut request: Request, next: Next) -> Response {
    let request_id = ["x-request-id", "requestid", "qid"]
        .iter()
        .find_map(|name| request.headers().get(*name))
        .and_then(|value| value.to_str().ok())
        .filter(|value| valid(value))
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| Uuid::new_v4().simple().to_string());

    request
        .extensions_mut()
        .insert(RequestId(request_id.clone()));
    let mut response = next.run(request).await;
    if let Ok(value) = HeaderValue::from_str(&request_id) {
        response
            .headers_mut()
            .insert(HeaderName::from_static("x-request-id"), value);
    }
    response
}

pub fn valid(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && !value.contains(['\r', '\n'])
        && value.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b':' | b'/')
        })
}

#[cfg(test)]
mod tests {
    use super::valid;

    #[test]
    fn rejects_injection_and_oversize_ids() {
        assert!(valid("K3S-accept_1:2/3"));
        assert!(!valid("bad\r\nheader"));
        assert!(!valid(&"a".repeat(65)));
        assert!(!valid("contains space"));
    }
}
