use crate::models::health::{HealthInfo, HealthStatus};

pub struct HealthService {}

impl HealthService {
    pub fn get_status() -> HealthInfo {
        HealthInfo {
            status: HealthStatus::OK,
        }
    }
}
