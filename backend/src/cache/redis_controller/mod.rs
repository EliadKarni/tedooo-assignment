use redis::{Client, Commands};
use once_cell::sync::OnceCell;
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;
use log::{info, debug, error};

/// A Redis-based caching utility.
///
/// This struct provides methods to interact with a Redis cache, including
/// setting and retrieving values with optional expiration times.
#[derive(Clone)]
pub struct RedisCache {
    client: OnceCell<Client>,
}

impl RedisCache {
    /// Creates a new RedisCache instance (lazy client initialization).
    pub async fn new() -> Result<Self, redis::RedisError> {
        info!("Initializing RedisCache (lazy)");
        Ok(RedisCache { client: OnceCell::new() })
    }

    fn get_or_init_client(&self) -> Result<&Client, redis::RedisError> {
        self.client.get_or_try_init(|| {
            let url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
            Client::open(url)
        })
    }

    /// Retrieves a value from the Redis cache.
    ///
    /// # Arguments
    /// - `key`: The key to retrieve the value for.
    ///
    /// # Returns
    /// An optional deserialized value of type `T`.
    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        debug!("Connecting to Redis to get key: {}", key);
        let client = match self.get_or_init_client() {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to initialize Redis client: {:?}", e);
                return None;
            }
        };
        let mut conn = match client.get_connection() {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to get Redis connection");
                return None;
            }
        };
        let value: Option<String> = match conn.get(key) {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to get value from Redis for key {}: {:?}", key, e);
                return None;
            }
        };
        match value {
            Some(ref v) => match serde_json::from_str(v) {
                Ok(deserialized) => Some(deserialized),
                Err(e) => {
                    error!("Failed to deserialize value from Redis for key {}: {:?}", key, e);
                    None
                }
            },
            None => {
                debug!("Key not found in Redis: {}", key);
                None
            }
        }
    }

    /// Sets a value in the Redis cache with an expiration time.
    ///
    /// # Arguments
    /// - `key`: The key to set the value for.
    /// - `value`: The value to set.
    /// - `ttl`: The time-to-live for the key.
    pub async fn set<T: Serialize>(&self, key: &str, value: &T, ttl: Duration) {
        debug!("Setting value in Redis for key {} with ttl {}", key, ttl.as_secs());
        let client = match self.get_or_init_client() {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to initialize Redis client: {:?}", e);
                return;
            }
        };
        let mut conn = match client.get_connection() {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to get Redis connection for set: {:?}", e);
                return;
            }
        };
        let value = match serde_json::to_string(value) {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to serialize value for Redis for key {}: {:?}", key, e);
                return;
            }
        };
        match {
            let res: Result<(), _> = conn.set_ex(key, value, (ttl.as_secs() as usize).try_into().unwrap());
            res
        } {
            Ok(_) => debug!("Successfully set value in Redis for key {}", key),
            Err(e) => error!("Failed to set value in Redis for key {}: {:?}", key, e),
        }
    }
}
