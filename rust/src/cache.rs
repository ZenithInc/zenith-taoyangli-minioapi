use async_trait::async_trait;
use redis::aio::ConnectionManager;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("cache operation failed")]
pub struct CacheError;

#[async_trait]
pub trait CacheStore: Send + Sync {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, CacheError>;
    async fn set_ex(&self, key: &str, value: &[u8], seconds: u64) -> Result<(), CacheError>;
    async fn try_lock(&self, key: &str, owner: &str, seconds: u64) -> Result<bool, CacheError>;
    async fn unlock(&self, key: &str, owner: &str) -> Result<(), CacheError>;
    async fn ping(&self) -> Result<(), CacheError>;
}

#[derive(Clone)]
pub struct RedisCache {
    manager: ConnectionManager,
}

impl RedisCache {
    pub async fn connect(url: &str) -> Result<Self, CacheError> {
        let client = redis::Client::open(url).map_err(|_| CacheError)?;
        let manager = ConnectionManager::new(client)
            .await
            .map_err(|_| CacheError)?;
        Ok(Self { manager })
    }
}

#[async_trait]
impl CacheStore for RedisCache {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, CacheError> {
        let mut connection = self.manager.clone();
        redis::cmd("GET")
            .arg(key)
            .query_async(&mut connection)
            .await
            .map_err(|_| CacheError)
    }

    async fn set_ex(&self, key: &str, value: &[u8], seconds: u64) -> Result<(), CacheError> {
        let mut connection = self.manager.clone();
        redis::cmd("SETEX")
            .arg(key)
            .arg(seconds)
            .arg(value)
            .query_async(&mut connection)
            .await
            .map_err(|_| CacheError)
    }

    async fn try_lock(&self, key: &str, owner: &str, seconds: u64) -> Result<bool, CacheError> {
        let mut connection = self.manager.clone();
        let result: Option<String> = redis::cmd("SET")
            .arg(key)
            .arg(owner)
            .arg("NX")
            .arg("EX")
            .arg(seconds)
            .query_async(&mut connection)
            .await
            .map_err(|_| CacheError)?;
        Ok(result.as_deref() == Some("OK"))
    }

    async fn unlock(&self, key: &str, owner: &str) -> Result<(), CacheError> {
        const COMPARE_DELETE: &str = "if redis.call('get', KEYS[1]) == ARGV[1] then return redis.call('del', KEYS[1]) else return 0 end";
        let mut connection = self.manager.clone();
        redis::cmd("EVAL")
            .arg(COMPARE_DELETE)
            .arg(1)
            .arg(key)
            .arg(owner)
            .query_async(&mut connection)
            .await
            .map_err(|_| CacheError)
    }

    async fn ping(&self) -> Result<(), CacheError> {
        let mut connection = self.manager.clone();
        let response: String = redis::cmd("PING")
            .query_async(&mut connection)
            .await
            .map_err(|_| CacheError)?;
        if response == "PONG" {
            Ok(())
        } else {
            Err(CacheError)
        }
    }
}
