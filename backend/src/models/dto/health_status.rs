use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HealthStatus {
    pub database: bool,
    pub redis: bool,
}