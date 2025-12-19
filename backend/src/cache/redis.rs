use redis::{AsyncCommands, Client};
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;

/// A Redis-based caching utility.
///
/// This struct provides methods to interact with a Redis cache, including
/// setting and retrieving values with optional expiration times.
pub struct RedisCache {
    client: Client,
}

impl RedisCache {
    /// Creates a new RedisCache instance.
    ///
    /// # Arguments
    /// - `redis_url`: The URL of the Redis server.
    pub fn new(redis_url: &str) -> Self {
        let client = Client::open(redis_url).expect("Failed to create Redis client");
        RedisCache { client }
    }

    /// Retrieves a value from the Redis cache.
    ///
    /// # Arguments
    /// - `key`: The key to retrieve the value for.
    ///
    /// # Returns
    /// An optional deserialized value of type `T`.
    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        let mut conn = self.client.get_async_connection().await.ok()?;
        let value: Option<String> = conn.get(key).await.ok()?;
        value.and_then(|v| serde_json::from_str(&v).ok())
    }

    /// Sets a value in the Redis cache with an expiration time.
    ///
    /// # Arguments
    /// - `key`: The key to set the value for.
    /// - `value`: The value to set.
    /// - `ttl`: The time-to-live for the key.
    pub async fn set<T: Serialize>(&self, key: &str, value: &T, ttl: Duration) {
        let mut conn = self.client.get_async_connection().await.expect("Failed to connect to Redis");
        let value = serde_json::to_string(value).expect("Failed to serialize value");
        let _: () = conn.set_ex(key, value, ttl.as_secs() as usize).await.expect("Failed to set value in Redis");
    }
}