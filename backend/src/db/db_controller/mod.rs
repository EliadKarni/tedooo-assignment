use sqlx::MySqlPool;
use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
use std::env;
use std::time::Duration;
use tokio::time::timeout;

use crate::utils::read_env_or_file;
use log::{debug, info, warn};


#[derive(Clone)]
pub struct MySQLController {
    pool: MySqlPool,
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
    pub async fn new() -> Result<Self, DbError> {
        debug!("[MySQLController::new] Reading DB connection parameters...");
        let host = env::var("DB_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port: u16 = env::var("DB_PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(3306);
        let db = env::var("DB_NAME").unwrap_or_else(|_| "todooo_db".to_string());
        debug!("[MySQLController::new] host: {}, port: {}, db: {}", host, port, db);
        let user = read_env_or_file("DB_USER").map_err(|e| sqlx::Error::Configuration(e.into()))?;
        let pass = read_env_or_file("DB_PASS").map_err(|e| sqlx::Error::Configuration(e.into()))?;
        debug!("[MySQLController::new] user: {}, pass: [REDACTED]", user);
        let opts: MySqlConnectOptions = MySqlConnectOptions::new()
            .host(&host)
            .port(port)
            .username(&user)
            .password(&pass)
            .database(&db);
        debug!("[MySQLController::new] Creating connection pool...");
        let pool = MySqlPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(Duration::from_secs(3))
            .connect_lazy_with(opts);
        info!("[MySQLController::new] Pool created (lazy)");
        Ok(Self {
            pool,
            default_query_timeout: Duration::from_secs(5),
        })
    }

    pub async fn exec(&self, sql: &str) -> Result<(), DbError> {
        self.exec_with_timeout(sql, self.default_query_timeout).await
    }

    pub async fn exec_with_timeout(&self, sql: &str, dur: Duration) -> Result<(), DbError> {
        debug!("[MySQLController::exec_with_timeout] Executing SQL: {} (timeout: {:?})", sql, dur);
        let mut conn = self.pool.acquire().await?;
        let fut = sqlx::query(sql).execute(&mut *conn);
        match timeout(dur, fut).await {
            Ok(res) => {
                debug!("[MySQLController::exec_with_timeout] Query completed");
                res?; // sqlx::Error -> DbError::Sqlx
                Ok(())
            }
            Err(_) => {
                warn!("[MySQLController::exec_with_timeout] Query timed out");
                let _ = conn.close().await;
                Err(DbError::Timeout)
            }
        }
    }
}
