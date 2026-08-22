use std::sync::Arc;

use taoyangli_tools::{
    AppState,
    cache::{CacheStore, RedisCache},
    config::Config,
    object_store::{ObjectStore, S3ObjectStore},
    token::TokenService,
    wechat::HttpWechatApi,
};
use tokio::{net::TcpListener, signal, sync::Semaphore};
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_current_span(false)
        .with_span_list(false)
        .init();

    let config = Config::from_env()?;
    let cache = Arc::new(RedisCache::connect(&config.redis_url).await?);
    let objects = Arc::new(S3ObjectStore::new(&config)?);
    if std::env::args().nth(1).as_deref() == Some("dependency-check") {
        let (redis, object_storage) = tokio::join!(cache.ping(), objects.health());
        let healthy = redis.is_ok() && object_storage.is_ok();
        println!(
            "{}",
            serde_json::json!({
                "status": if healthy { "ok" } else { "failed" },
                "checks": {
                    "redis": {"status": if redis.is_ok() { "ok" } else { "failed" }},
                    "object_storage": {"status": if object_storage.is_ok() { "ok" } else { "failed" }}
                }
            })
        );
        if healthy {
            return Ok(());
        }
        std::process::exit(1);
    }
    let wechat = Arc::new(HttpWechatApi::new(config.wechat_api_base_url.clone())?);
    let tokens = TokenService::new(cache.clone(), wechat, config.token_mode, &config.app_name);
    let state = AppState {
        tokens,
        cache,
        objects,
        uploads: Arc::new(Semaphore::new(2)),
        temp_dir: config.temp_dir.clone(),
    };
    let listener = TcpListener::bind(config.listen).await?;
    info!(listen = %config.listen, token_mode = ?config.token_mode, "tools service started");
    axum::serve(
        listener,
        taoyangli_tools::app(state).into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        let _ = signal::ctrl_c().await;
    };
    #[cfg(unix)]
    let terminate = async {
        if let Ok(mut signal) = signal::unix::signal(signal::unix::SignalKind::terminate()) {
            signal.recv().await;
        }
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }
}
