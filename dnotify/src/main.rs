use crate::client::DNotify;

mod client;

/// Main startup function.
#[tokio::main]
async fn main() {
    DNotify::start().await;
}
