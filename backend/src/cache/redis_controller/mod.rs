use log::{debug, info};
use redis::aio::ConnectionManager;
use redis::AsyncCommands;
use serde::{de::DeserializeOwned, Serialize};
use std::{sync::Arc, time::Duration};
use thiserror::Error;
use tokio::sync::OnceCell;


#[derive(Clone)]
pub struct RedisCache {
    cm: Arc<OnceCell<ConnectionManager>>,
    op_timeout: Duration,
}

impl RedisCache {
    /// No I/O here -> no need for async / Result.
    pub fn new() -> Self {
        info!("Initializing RedisCache (lazy ConnectionManager)...");
        Self {
            cm: Arc::new(OnceCell::new()),
            op_timeout: Duration::from_millis(300),
        }
    }

    async fn manager(&self) -> Result<ConnectionManager, CacheError> {
        self.manager_with_timeout(Duration::from_millis(300)).await
    }

    /// Lazily initializes and returns a clone of the Redis `ConnectionManager`.
    ///
    /// The connection manager is created only once (via `OnceCell`) and reused afterwards.
    /// If initialization fails, the `OnceCell` remains empty, and the next call will retry.
    async fn manager_with_timeout(&self, timeout: Duration) -> Result<ConnectionManager, CacheError> {
        let cm_ref = self
            .cm
            .get_or_try_init(|| async {
                // Build a valid Redis URL from CACHE_HOST + CACHE_PORT (+ CACHE_DB).
                // CACHE_HOST may be either:
                // - a plain host/domain/IP (e.g. "redis" / "127.0.0.1")
                // - a full URL (e.g. "redis://redis:6379/0")
                let url = Self::build_redis_url_from_env();

                info!("Redis URL: {}", Self::redact_redis_url(&url));

                let client = redis::Client::open(url)?;
                let cm = tokio::time::timeout(timeout, ConnectionManager::new(client))
                    .await.map_err(|_| CacheError::Timeout)??;

                Ok::<ConnectionManager, CacheError>(cm)
            })
            .await?;

        Ok(cm_ref.clone())
    }

    /// Builds a Redis connection URL from host/domain + port (+ db index).
    ///
    /// Supported inputs:
    /// - `CACHE_HOST` as plain host/domain/IP -> "redis://{host}:{port}/{db}"
    /// - `CACHE_HOST` as full URL ("redis://..." or "rediss://...") -> returned as-is
    ///   (best-effort to append `/{db}` if no path exists).
    fn build_redis_url(host: &str, port: u16, db: u8) -> String {
        let host = host.trim();

        // If the user provided a full URL, keep it.
        if host.starts_with("redis://") || host.starts_with("rediss://") {
            // If it already contains a path, leave it untouched.
            if host.contains('/') {
                return host.to_string();
            }
            // No path -> append db index.
            return format!("{host}/{db}");
        }

        // Plain hostname/IP -> build a proper Redis URL.
        format!("redis://{host}:{port}/{db}")
    }

    /// Redacts credentials from a Redis URL for safe logging.
    ///
    /// Example:
    /// - Input:  "redis://:secret@redis:6379/0"
    /// - Output: "redis://***@redis:6379/0"
    fn redact_redis_url(url: &str) -> String {
        if let Some(at) = url.find('@') {
            if let Some(scheme_end) = url.find("://") {
                return format!("{}***@{}", &url[..scheme_end + 3], &url[at + 1..]);
            }
        }
        url.to_string()
    }

    /// Reads an environment variable and parses it as `u16`.
    ///
    /// - Trims whitespace
    /// - Falls back to `default_val` on missing/invalid values
    fn read_u16_env(key: &str, default_val: u16) -> u16 {
        std::env::var(key)
            .ok()
            .and_then(|s| s.trim().parse::<u16>().ok())
            .unwrap_or(default_val)
    }

    /// Reads an environment variable and parses it as `u8`.
    ///
    /// - Trims whitespace
    /// - Falls back to `default_val` on missing/invalid values
    fn read_u8_env(key: &str, default_val: u8) -> u8 {
        std::env::var(key)
            .ok()
            .and_then(|s| s.trim().parse::<u8>().ok())
            .unwrap_or(default_val)
    }


    /// Builds the Redis URL using:
    /// - CACHE_HOST (host/domain/IP or full URL)
    /// - CACHE_PORT (defaults to 6379)
    /// - CACHE_DB   (defaults to 0)
    pub fn build_redis_url_from_env() -> String {
        let host = std::env::var("CACHE_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = Self::read_u16_env("CACHE_PORT", 6379);
        let db = Self::read_u8_env("CACHE_DB", 0);

        Self::build_redis_url(&host, port, db)
    }

    /// "Proper" API: distinguish between miss and errors.
    pub async fn get<T: DeserializeOwned>(
        &self,
        key: &str,
    ) -> Result<Option<T>, CacheError> {
        debug!("Redis GET key={}", key);
        let mut cm = self.manager().await?;

        let raw: Option<String> = tokio::time::timeout(
            self.op_timeout,
            cm.get::<_, Option<String>>(key),
        )
        .await
        .map_err(|_| CacheError::Timeout)??;

        match raw {
            Some(s) => Ok(Some(serde_json::from_str::<T>(&s)?)),
            None => Ok(None),
        }
    }

    pub async fn set<T: Serialize>(
        &self,
        key: &str,
        value: &T,
        ttl: Duration,
    ) -> Result<(), CacheError> {
        debug!("Redis SETEX key={} ttl={:?}", key, ttl);
        let mut cm = self.manager().await?;

        let s = serde_json::to_string(value)?;
        let seconds: u64 = ttl.as_secs();

        // IMPORTANT: await the timeout future, then propagate both layers of Result.
        tokio::time::timeout(
            self.op_timeout,
            cm.set_ex::<_, _, ()>(key, s, seconds),
        )
        .await
        .map_err(|_| CacheError::Timeout)??;

        Ok(())
    }

    pub async fn is_available(&self) -> bool {
        match self.ping().await {
            Ok(()) => true,
            Err(e) => {
                debug!("Redis healthcheck failed: {}", e);
                false
            }
        }
    }

    /// Performs a Redis PING command and validates the response.
    pub async fn ping(&self) -> Result<(), CacheError> {
        let mut cm = self.manager_with_timeout(Duration::from_secs(1)).await?;

        let pong: String = tokio::time::timeout(Duration::from_secs(1), cm.ping())
            .await
            .map_err(|_| CacheError::Timeout)??;

        if pong == "PONG" {
            Ok(())
        } else {
            Err(CacheError::UnexpectedResponse(pong))
        }
    }
}

#[derive(Error, Debug)]
pub enum CacheError {
    #[error("redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("timeout")]
    Timeout,

    #[error("unexpected response: {0}")]
    UnexpectedResponse(String),
}
