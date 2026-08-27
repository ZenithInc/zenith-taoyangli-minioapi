use async_trait::async_trait;
use aws_credential_types::Credentials;
use aws_sdk_s3::{Client, config::BehaviorVersion, primitives::ByteStream};
use aws_types::region::Region;
use std::path::Path;
use thiserror::Error;

use crate::config::Config;

#[derive(Debug, Error)]
#[error("object storage operation failed")]
pub struct ObjectStoreError;

#[async_trait]
pub trait ObjectStore: Send + Sync {
    async fn put(
        &self,
        path: &Path,
        object_name: &str,
        content_type: &str,
    ) -> Result<String, ObjectStoreError>;
    async fn health(&self) -> Result<(), ObjectStoreError>;
}

#[derive(Clone)]
pub struct S3ObjectStore {
    client: Client,
    bucket: String,
    public_base_url: url::Url,
}

impl S3ObjectStore {
    pub fn new(config: &Config) -> Result<Self, ObjectStoreError> {
        let credentials = Credentials::new(
            config.s3_access_key.clone(),
            config.s3_secret_key.clone(),
            None,
            None,
            "taoyangli-tools-env",
        );
        let s3_config = aws_sdk_s3::Config::builder()
            .behavior_version(BehaviorVersion::latest())
            .region(Region::new(config.s3_region.clone()))
            .credentials_provider(credentials)
            .endpoint_url(config.s3_endpoint.clone())
            .force_path_style(true)
            .build();
        Ok(Self {
            client: Client::from_conf(s3_config),
            bucket: config.s3_bucket.clone(),
            public_base_url: url::Url::parse(&config.s3_public_base_url)
                .map_err(|_| ObjectStoreError)?,
        })
    }

    fn public_url(&self, object_name: &str) -> Result<String, ObjectStoreError> {
        let mut url = self.public_base_url.clone();
        url.path_segments_mut()
            .map_err(|_| ObjectStoreError)?
            .pop_if_empty()
            .push(object_name);
        Ok(url.into())
    }
}

#[async_trait]
impl ObjectStore for S3ObjectStore {
    async fn put(
        &self,
        path: &Path,
        object_name: &str,
        content_type: &str,
    ) -> Result<String, ObjectStoreError> {
        let body = ByteStream::from_path(path)
            .await
            .map_err(|_| ObjectStoreError)?;
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(object_name)
            .content_type(content_type)
            .body(body)
            .send()
            .await
            .map_err(|_| ObjectStoreError)?;
        self.public_url(object_name)
    }

    async fn health(&self) -> Result<(), ObjectStoreError> {
        self.client
            .head_bucket()
            .bucket(&self.bucket)
            .send()
            .await
            .map_err(|_| ObjectStoreError)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::config::{Config, TokenMode};

    use super::S3ObjectStore;

    #[test]
    fn constructs_s3_client_with_an_explicit_behavior_version() {
        let config = Config {
            listen: "127.0.0.1:8080".parse().unwrap(),
            app_name: "tools-test".to_owned(),
            token_mode: TokenMode::ShadowReadonly,
            redis_url: "redis://127.0.0.1:6379/0".to_owned(),
            wechat_api_base_url: "https://api.weixin.qq.com".to_owned(),
            s3_endpoint: "https://objects.example.test".to_owned(),
            s3_region: "cn-north-1".to_owned(),
            s3_access_key: "test-access-key".to_owned(),
            s3_secret_key: "test-secret-key".to_owned(),
            s3_bucket: "test-bucket".to_owned(),
            s3_public_base_url: "https://objects.example.test/test-bucket/".to_owned(),
            temp_dir: PathBuf::from("/tmp"),
        };

        S3ObjectStore::new(&config).expect("S3 client configuration should be valid");
    }
}
