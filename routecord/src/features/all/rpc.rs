use crate::config::Config;
use std::sync::Arc;
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System, UpdateKind};
use yet_another_discord_rpc::DiscordRpc;

pub struct DiscordRPC {
    /// RPC Instance.
    rpc: DiscordRpc,

    /// Cached config instance.
    config: Arc<Config>,

    /// `sysinfo` System instance so we don't have to always
    /// reconstruct it.
    system: System,

    /// Last-set RPC JSON Map.
    last_set_rpc: Option<serde_json::Map<String, serde_json::Value>>,
}

impl DiscordRPC {
    pub async fn new(
        config: Arc<Config>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Self {
            rpc: DiscordRpc::new("1453758039403401216").await?,
            config,
            system: System::new_with_specifics(
                RefreshKind::nothing()
                    .with_processes(ProcessRefreshKind::nothing().with_user(UpdateKind::Always)),
            ),
            last_set_rpc: None,
        })
    }

    /// Starts the process watcher, responsible for syncing the RPC.
    ///
    /// **Note**: Should be called on a new task as it spawns an infinite loop.
    pub async fn start_process_watcher(&mut self) {
        self.system.refresh_processes(ProcessesToUpdate::All, true);
        self.rpc
            .start_activity(Some(serde_json::Value::Null))
            .await
            .expect("[ERROR] Failed starting empty RPC activity!");

        let mut has_rpc;

        loop {
            has_rpc = false;
            self.system.refresh_processes(ProcessesToUpdate::All, true);
            let process_names: Vec<String> = self
                .system
                .processes()
                .values()
                .filter_map(|process| {
                    let process_name = process
                        .name()
                        .to_str()
                        .expect("[ERROR] Failed calling to_str() on process name!");
                    if self.config.get_config_rpc_value(process_name).is_some() {
                        return Some(process_name.to_owned());
                    }

                    None
                })
                .collect();

            for process_name in process_names {
                if self.try_sync_rpc_to_process(&process_name).await {
                    has_rpc = true;

                    // Only one RPC can be applied per instance, hence this
                    // break-call or we'd be cycling statuses constantly, if
                    // there's an upcoming one.
                    break;
                }
            }

            if !has_rpc {
                let _ = self.rpc.clear_activity().await;
                self.last_set_rpc = None;
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    }

    /// Tries to find `key` inside of `target`, then adding it
    /// to `map` by cloning the value.
    fn clone_kv_if_some(
        &self,
        key: &str,
        target: &serde_json::Value,
        map: &mut serde_json::Map<String, serde_json::Value>,
    ) -> bool {
        if let Some(value) = target.get(key) {
            map.insert(key.to_owned(), value.to_owned());
            return true;
        }

        false
    }

    /// Tries to sync the RPC to the given process name, assuming
    /// there's an entry for it inside of config.
    async fn try_sync_rpc_to_process(&mut self, process_name: &str) -> bool {
        let Some(process_config) = self.config.get_config_rpc_value(process_name) else {
            return false;
        };

        let mut should_update_rpc = false;
        let mut main_map = serde_json::Map::with_capacity(4);
        let mut assets_map = serde_json::Map::with_capacity(4);

        // `name` must always be present in order for the RPC to update.
        if self.clone_kv_if_some("name", process_config, &mut main_map) {
            should_update_rpc = true;
        }

        if !should_update_rpc {
            eprintln!(
                "[WARN] RPC won't update for \"{process_name}\" due to it lacking the `name` key."
            );
            return false;
        }

        self.clone_kv_if_some("details", process_config, &mut main_map);
        self.clone_kv_if_some("state", process_config, &mut main_map);
        self.clone_kv_if_some("type", process_config, &mut main_map);

        if let Some(assets_config) = process_config.get("assets") {
            self.clone_kv_if_some("large_image", assets_config, &mut assets_map);
            self.clone_kv_if_some("large_text", assets_config, &mut assets_map);

            self.clone_kv_if_some("small_image", assets_config, &mut assets_map);
            self.clone_kv_if_some("small_text", assets_config, &mut assets_map);
            main_map.insert("assets".to_owned(), assets_map.into());
        }

        if let Some(last_set_rpc) = &self.last_set_rpc
            && *last_set_rpc == main_map
        {
            return true;
        }

        self.last_set_rpc = Some(main_map.to_owned());
        if let Err(error) = self.rpc.set_activity(main_map.into()).await {
            eprintln!(
                "[ERROR] Failed syncing RPC activity to process \"{process_name}\", error: {error}"
            );
            return false;
        }

        true
    }
}
