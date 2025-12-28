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
    let enable_notifications = config.enable_notifications();

    if config.enable_rpc() {
        let mut rpc = DiscordRPC::new(config_clone)
            .await
            .expect("[ERROR] Failed starting Discord RPC!");

        if !enable_notifications {
            rpc.start_process_watcher().await;
        } else {
            tokio::task::spawn(async move {
                rpc.start_process_watcher().await;
            });
        }
    }

    if !enable_notifications {
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    #[cfg(target_os = "windows")]
    crate::features::windows::discord_client::DNotify::start(config);

    #[cfg(target_os = "linux")]
    crate::features::linux::discord_client::DNotify::start(config).await;
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
