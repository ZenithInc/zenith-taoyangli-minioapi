use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("WeChat API request failed")]
pub struct WechatError;

#[async_trait]
pub trait WechatApi: Send + Sync {
    async fn get(&self, path: &str, query: &HashMap<&str, String>) -> Result<Value, WechatError>;
}

#[derive(Clone)]
pub struct HttpWechatApi {
    client: reqwest::Client,
    base_url: String,
}

impl HttpWechatApi {
    pub fn new(base_url: String) -> Result<Self, WechatError> {
        let client = reqwest::Client::builder()
            .connect_timeout(std::time::Duration::from_secs(5))
            .timeout(std::time::Duration::from_secs(15))
            .build()
            .map_err(|_| WechatError)?;
        Ok(Self {
            client,
            base_url: base_url.trim_end_matches('/').to_owned(),
        })
    }
}

#[async_trait]
impl WechatApi for HttpWechatApi {
    async fn get(&self, path: &str, query: &HashMap<&str, String>) -> Result<Value, WechatError> {
        let response = self
            .client
            .get(format!("{}{}", self.base_url, path))
            .query(query)
            .send()
            .await
            .map_err(|_| WechatError)?;
        if !response.status().is_success() {
            return Err(WechatError);
        }
        response.json().await.map_err(|_| WechatError)
    }
}
