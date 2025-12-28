use crate::{config::Config, features::all::rpc::DiscordRPC};
use std::sync::Arc;

mod config;
mod features;
mod utils;

/// Main startup function.
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let config = Arc::new(Config::new().expect("[ERROR] Failed creating Config instance!"));
    let config_clone = Arc::clone(&config);

    if config.enable_rpc() {
        tokio::task::spawn(async move {
            let mut rpc = DiscordRPC::new(config_clone)
                .await
                .expect("[ERROR] Failed starting Discord RPC!");
            rpc.start_process_watcher().await;
        });
    }

    if !config.enable_notifications() {
        return;
    }

    #[cfg(target_os = "windows")]
    crate::features::windows::discord_client::DNotify::start(config);

    #[cfg(target_os = "linux")]
    crate::features::linux::discord_client::DNotify::start(config).await;
}
