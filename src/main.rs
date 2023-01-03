use tracing::info;
use tracing_subscriber;

fn main() {
    tracing_subscriber::fmt::init();
    info!("Hello, world!");
}
