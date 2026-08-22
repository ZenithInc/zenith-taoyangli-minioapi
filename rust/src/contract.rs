use axum::{Json, http::HeaderMap};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;

pub const SUCCESS_CODE: i64 = 100_000;
pub const BAD_REQUEST_CODE: i64 = 100_010;
pub const SERVER_ERROR_CODE: i64 = 100_014;
pub const UPLOAD_ERROR_CODE: i64 = 600;

#[derive(Debug, Error)]
#[error("request parameters are malformed")]
pub struct ParseParamsError;

#[derive(Debug, Serialize)]
pub struct LegacyResponse {
    #[serde(rename = "requestId")]
    pub request_id: String,
    pub return_code: i64,
    pub msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl LegacyResponse {
    pub fn success(request_id: String, msg: impl Into<String>, data: Value) -> Json<Self> {
        Json(Self {
            request_id,
            return_code: SUCCESS_CODE,
            msg: msg.into(),
            data: Some(data),
        })
    }

    pub fn failure(request_id: String, code: i64, msg: impl Into<String>) -> Json<Self> {
        Json(Self {
            request_id,
            return_code: code,
            msg: msg.into(),
            data: None,
        })
    }
}

pub fn parse_params(
    headers: &HeaderMap,
    body: &[u8],
) -> Result<HashMap<String, String>, ParseParamsError> {
    let content_type = headers
        .get(axum::http::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();

    if content_type.starts_with("application/json") {
        serde_json::from_slice(body).map_err(|_| ParseParamsError)
    } else {
        serde_urlencoded::from_bytes(body).map_err(|_| ParseParamsError)
    }
}
