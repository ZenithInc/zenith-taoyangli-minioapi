use std::{env, net::SocketAddr, path::PathBuf};

use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenMode {
    Active,
    ShadowReadonly,
}

impl TokenMode {
    fn parse(value: &str) -> Result<Self, ConfigError> {
        match value {
            "active" => Ok(Self::Active),
            "shadow-readonly" => Ok(Self::ShadowReadonly),
            _ => Err(ConfigError::Invalid("TOOLS_TOKEN_MODE")),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    pub listen: SocketAddr,
    pub app_name: String,
    pub token_mode: TokenMode,
    pub redis_url: String,
    pub wechat_api_base_url: String,
    pub s3_endpoint: String,
    pub s3_region: String,
    pub s3_access_key: String,
    pub s3_secret_key: String,
    pub s3_bucket: String,
    pub s3_public_base_url: String,
    pub temp_dir: PathBuf,
    pub upload_bearer_token: String,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("required environment variable is missing: {0}")]
    Missing(&'static str),
    #[error("invalid environment variable: {0}")]
    Invalid(&'static str),
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let host = env::var("TOOLS_LISTEN_HOST").unwrap_or_else(|_| "0.0.0.0".to_owned());
        let port = env::var("TOOLS_LISTEN_PORT")
            .unwrap_or_else(|_| "8080".to_owned())
            .parse::<u16>()
            .map_err(|_| ConfigError::Invalid("TOOLS_LISTEN_PORT"))?;
        let listen = format!("{host}:{port}")
            .parse()
            .map_err(|_| ConfigError::Invalid("TOOLS_LISTEN_HOST"))?;

        let redis_host = required("CACHE_REDIS_HOST")?;
        let redis_port = env::var("CACHE_REDIS_PORT")
            .unwrap_or_else(|_| "6379".to_owned())
            .parse::<u16>()
            .map_err(|_| ConfigError::Invalid("CACHE_REDIS_PORT"))?;
        let redis_db = env::var("CACHE_REDIS_DB")
            .unwrap_or_else(|_| "0".to_owned())
            .parse::<u32>()
            .map_err(|_| ConfigError::Invalid("CACHE_REDIS_DB"))?;
        let redis_auth = env::var("CACHE_REDIS_AUTH").unwrap_or_default();
        let redis_url = redis_url(&redis_host, redis_port, redis_db, &redis_auth)?;

        let s3_public_base_url = normalize_public_base_url(&required("S3_PUBLIC_BASE_URL")?)?;
        let wechat_api_base_url = required("WECHAT_API_BASE_URL")?;
        url::Url::parse(&wechat_api_base_url)
            .map_err(|_| ConfigError::Invalid("WECHAT_API_BASE_URL"))?;
        let s3_endpoint = required("S3_ENDPOINT")?;
        url::Url::parse(&s3_endpoint).map_err(|_| ConfigError::Invalid("S3_ENDPOINT"))?;

        let upload_bearer_token = required("TOOLS_UPLOAD_BEARER_TOKEN")?;
        if upload_bearer_token.len() < 32 {
            return Err(ConfigError::Invalid("TOOLS_UPLOAD_BEARER_TOKEN"));
        }

        Ok(Self {
            listen,
            app_name: required("APP_NAME")?,
            token_mode: TokenMode::parse(&required("TOOLS_TOKEN_MODE")?)?,
            redis_url,
            wechat_api_base_url,
            s3_endpoint,
            s3_region: required("S3_REGION")?,
            s3_access_key: required("S3_ACCESS_KEY")?,
            s3_secret_key: required("S3_SECRET_KEY")?,
            s3_bucket: required("S3_BUCKET")?,
            s3_public_base_url,
            temp_dir: PathBuf::from(
                env::var("TOOLS_TMP_DIR").unwrap_or_else(|_| "/tmp".to_owned()),
            ),
            upload_bearer_token,
        })
    }
}

fn required(key: &'static str) -> Result<String, ConfigError> {
    env::var(key)
        .ok()
        .filter(|value| !value.is_empty())
        .ok_or(ConfigError::Missing(key))
}

fn redis_url(host: &str, port: u16, db: u32, password: &str) -> Result<String, ConfigError> {
    let mut url = url::Url::parse("redis://localhost")
        .map_err(|_| ConfigError::Invalid("CACHE_REDIS_HOST"))?;
    url.set_host(Some(host))
        .map_err(|_| ConfigError::Invalid("CACHE_REDIS_HOST"))?;
    url.set_port(Some(port))
        .map_err(|_| ConfigError::Invalid("CACHE_REDIS_PORT"))?;
    url.set_path(&format!("/{db}"));
    if !password.is_empty() {
        url.set_password(Some(password))
            .map_err(|_| ConfigError::Invalid("CACHE_REDIS_AUTH"))?;
    }
    Ok(url.into())
}

fn normalize_public_base_url(value: &str) -> Result<String, ConfigError> {
    let mut url = url::Url::parse(value).map_err(|_| ConfigError::Invalid("S3_PUBLIC_BASE_URL"))?;
    match url.scheme() {
        "http" => url
            .set_scheme("https")
            .map_err(|_| ConfigError::Invalid("S3_PUBLIC_BASE_URL"))?,
        "https" => {}
        _ => return Err(ConfigError::Invalid("S3_PUBLIC_BASE_URL")),
    }
    Ok(url.into())
}

#[cfg(test)]
mod tests {
    use super::{normalize_public_base_url, redis_url};

    #[test]
    fn redis_password_is_percent_encoded() {
        let url = redis_url("redis.internal", 6380, 3, "p@ss:/word").unwrap();
        assert_eq!(url, "redis://:p%40ss%3A%2Fword@redis.internal:6380/3");
    }

    #[test]
    fn public_upload_urls_preserve_legacy_https_normalization() {
        assert_eq!(
            normalize_public_base_url("http://objects.example.test/bucket/").unwrap(),
            "https://objects.example.test/bucket/"
        );
        assert!(normalize_public_base_url("ftp://objects.example.test/").is_err());
    }
}
