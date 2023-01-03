#[derive(Debug)]
pub enum HealthStatus {
    OK,
}

#[derive(Debug)]
pub struct HealthInfo {
    pub status: HealthStatus,
}
