#[cfg(target_os = "linux")]
use crate::client_linux::DNotify;

#[cfg(target_os = "windows")]
use crate::client_windows::DNotify;

#[cfg(target_os = "linux")]
mod client_linux;

#[cfg(target_os = "windows")]
mod client_windows;

/// Main startup function.
#[tokio::main(flavor = "current_thread")]
async fn main() {
    DNotify::start().await;
}
