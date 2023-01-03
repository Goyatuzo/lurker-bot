mod services;
mod models;

use std::time::Duration;

use tokio::{task, time};
use tracing::info;
use tracing_subscriber;

use crate::services::health::HealthService;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let health_log = task::spawn(async {
        let mut interval = time::interval(Duration::from_secs(10));

        loop {
            interval.tick().await;

            info!("{:?}", HealthService::get_status());
        }
    });

    health_log.await.expect("Health check cannot err");
}
