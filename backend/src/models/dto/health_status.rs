use serde::Serialize;

#[derive(Serialize)]
pub struct HealthStatus {
    pub database: bool,
    pub redis: bool,
}