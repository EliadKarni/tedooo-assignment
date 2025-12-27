use futures::future::BoxFuture;
use log::{debug, info, warn};
use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
use sqlx::{MySql, MySqlConnection, MySqlPool};
use sqlx::pool::PoolConnection;
use std::{env, sync::Arc, time::Duration};
use tokio::sync::OnceCell;
use tokio::time::timeout;

use crate::utils::read_env_or_file;

#[derive(Clone)]
pub struct MySQLController {
    pool: Arc<OnceCell<MySqlPool>>,
    opts: MySqlConnectOptions,

    max_connections: u32,
    acquire_timeout: Duration,
    pool_init_timeout: Duration,
    default_query_timeout: Duration,
}

#[derive(thiserror::Error, Debug)]
pub enum DbError {
    #[error("db timeout")]
    Timeout,
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

impl MySQLController {
    /// Builds a controller without opening any network connections.
    /// The pool is created lazily on first use.
    pub fn new() -> Result<Self, DbError> {
        debug!("[MySQLController::new] Reading DB connection parameters...");
        let host = env::var("DB_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port: u16 = env::var("DB_PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(3306);
        let db = env::var("DB_NAME").unwrap_or_else(|_| "tedooo_db".to_string());

        let user = read_env_or_file("DB_USER").map_err(|e| sqlx::Error::Configuration(e.into()))?;
        let pass = read_env_or_file("DB_PASS").map_err(|e| sqlx::Error::Configuration(e.into()))?;

        let opts = MySqlConnectOptions::new()
            .host(&host)
            .port(port)
            .username(&user)
            .password(&pass)
            .database(&db);

        info!("[MySQLController::new] Controller created (lazy pool via OnceCell)");

        Ok(Self {
            pool: Arc::new(OnceCell::new()),
            opts,
            max_connections: 10,
            acquire_timeout: Duration::from_secs(3),
            pool_init_timeout: Duration::from_secs(3),
            default_query_timeout: Duration::from_secs(5),
        })
    }

    /// Returns a clone of the lazily-initialized pool.
    async fn pool(&self) -> Result<MySqlPool, DbError> {
        let pool_ref = self
            .pool
            .get_or_try_init(|| async {
                debug!("[MySQLController::pool] Initializing pool...");

                let fut = MySqlPoolOptions::new()
                    .max_connections(self.max_connections)
                    .acquire_timeout(self.acquire_timeout)
                    .connect_with(self.opts.clone()); // Future<Output = Result<MySqlPool, sqlx::Error>>

                // timeout -> DbError::Timeout
                // inner sqlx::Error -> DbError via #[from]
                let pool = timeout(self.pool_init_timeout, fut)
                    .await
                    .map_err(|_| DbError::Timeout)??;

                debug!("[MySQLController::pool] Pool initialized");

                Ok::<MySqlPool, DbError>(pool)
            })
            .await?;

        Ok(pool_ref.clone())
    }
    pub async fn exec(&self, sql: &str) -> Result<(), DbError> {
        self.exec_with_timeout(sql, self.default_query_timeout).await
    }

    pub async fn exec_with_timeout(&self, sql: &str, dur: Duration) -> Result<(), DbError> {
        debug!("[MySQLController::exec_with_timeout] Executing SQL: {} (timeout: {:?})", sql, dur);

        let pool = self.pool().await?;

        // Acquire connection with an explicit timeout as well.
        let mut conn = timeout(self.acquire_timeout, pool.acquire())
            .await
            .map_err(|_| DbError::Timeout)??;

        let fut = sqlx::query(sql).execute(&mut *conn);

        match timeout(dur, fut).await {
            Ok(res) => {
                res?;
                Ok(())
            }
            Err(_) => {
                warn!("[MySQLController::exec_with_timeout] Query timed out");
                let _ = conn.close().await;
                Err(DbError::Timeout)
            }
        }
    }

    pub async fn with_conn_timeout<T, F>(&self, dur: Duration, f: F) -> Result<T, DbError>
    where
        T: Send,
        F: for<'c> FnOnce(&'c mut MySqlConnection) -> BoxFuture<'c, Result<T, sqlx::Error>> + Send,
    {
        let pool = self.pool().await?;

        match timeout(dur, async {
            let mut pooled: PoolConnection<MySql> =
                timeout(self.acquire_timeout, pool.acquire())
                    .await
                    .map_err(|_| sqlx::Error::PoolTimedOut)??;

            f(pooled.as_mut()).await
        })
        .await
        {
            Ok(res) => Ok(res?),
            Err(_) => Err(DbError::Timeout),
        }
    }

    pub async fn with_conn<T, F>(&self, f: F) -> Result<T, DbError>
    where
        T: Send,
        F: for<'c> FnOnce(&'c mut MySqlConnection) -> BoxFuture<'c, Result<T, sqlx::Error>> + Send,
    {
        self.with_conn_timeout(self.default_query_timeout, f).await
    }

        /// Convenience: sensible default for health checks.
    pub async fn is_db_available(&self) -> bool {
        self.is_db_available_timeout(Duration::from_secs(1)).await
    }
    
     /// Returns `true` if the DB is reachable and can execute a trivial query.
    /// Uses the lazy pool initialization path (OnceCell) and bounds the whole check by `dur`.
    pub async fn is_db_available_timeout(&self, dur: Duration) -> bool {
        // Bound *everything* (pool init + acquire + query) by dur.
        let fut = async {
            // Trigger lazy pool init (or reuse existing pool)
            let pool = self.pool().await?;

            // Acquire a connection and run a trivial query.
            let mut conn = pool.acquire().await?;
            sqlx::query("SELECT 1").execute(&mut *conn).await?;
            Ok::<(), DbError>(())
        };

        match timeout(dur, fut).await {
            Ok(Ok(())) => true,
            _ => false,
        }
    }
   
}