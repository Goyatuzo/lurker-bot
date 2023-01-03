mod models;
mod services;

use std::time::Duration;

use dotenv::dotenv;
use serenity::{framework::StandardFramework, prelude::GatewayIntents, Client};
use tokio::{task, time};
use tracing::info;
use tracing_subscriber;

use crate::services::health::HealthService;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    task::spawn(async {
        let mut interval = time::interval(Duration::from_secs(10));

        loop {
            interval.tick().await;

            info!("{:?}", HealthService::get_status());
        }
    });

    //health_log.await.expect("Health check cannot err");

    let token = std::env::var("DISCORD_TOKEN").expect("Discord Token cannot be null");
    let intents = GatewayIntents::non_privileged();
    let framework = StandardFramework::new();

    let mut client = Client::builder(token, intents)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
