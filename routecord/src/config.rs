use crate::utils::Utils;
use anyhow::{Context, Result};

/// Fercord configuration system.
pub struct Config {
    /// Parsed `config.json`.
    config: serde_json::Value,

    /// Parsed `secrets.json`, holding the Token and User ID.
    secrets: serde_json::Value,
}

impl Config {
    /// Creates a new instance of `Config`.
    pub fn new() -> Result<Self> {
        let mut config_file_path = Utils::get_config_dir_path();
        let mut secrets_file_path = Utils::get_config_dir_path();

        config_file_path.push("config.json");
        secrets_file_path.push("secrets.json");

        let parsed_config = serde_json::from_slice(
            &std::fs::read(config_file_path).context("Failed reading config.json!")?,
        )
        .context("Failed parsing config.json!")?;

        let parsed_secrets = serde_json::from_slice(
            &std::fs::read(secrets_file_path).context("Failed reading secrets.json!")?,
        )
        .context("Failed parsing secrets.json!")?;

        Ok(Self {
            config: parsed_config,
            secrets: parsed_secrets,
        })
    }

    /// Gets the `enable_rpc` bool from `config.json`, crashing if it fails.
    pub fn enable_rpc(&self) -> bool {
        self.get_config_value("enable_rpc")
            .expect("[ERROR] Failed finding `enable_rpc` inside of config.json!")
            .as_bool()
            .expect("[ERROR] Failed turning `enable_rpc` from config.json into a bool!")
    }

    /// Gets the `enable_notifications` bool from `config.json`, crashing if it fails.
    pub fn enable_notifications(&self) -> bool {
        self.get_config_value("enable_notifications")
            .expect("[ERROR] Failed finding `enable_notifications` inside of config.json!")
            .as_bool()
            .expect("[ERROR] Failed turning `enable_notifications` from config.json into a bool!")
    }

    /// Gets the `token` string from `secrets.json`, crashing if it fails.
    pub fn get_token(&self) -> &str {
        self.get_secrets_value("token")
            .expect("[ERROR] Failed finding `token` inside of secrets.json!")
            .as_str()
            .expect("[ERROR] Failed turning `token` from secrets.json into a &str!")
    }

    /// Gets the `user_id` u64 from `secrets.json`, crashing if it fails.
    pub fn get_user_id(&self) -> u64 {
        self.get_secrets_value("user_id")
            .expect("[ERROR] Failed finding `token` inside of secrets.json!")
            .as_u64()
            .expect("[ERROR] Failed turning `token` from secrets.json into a u64!")
    }

    /// Tries to extract a value out of `config.json` » `rpc` » `key`.
    #[inline]
    pub fn get_config_rpc_value(&self, key: &str) -> Option<&serde_json::Value> {
        self.get_config_value("rpc")?.get(key)
    }

    /// Tries to extract a value out of `config.json`.
    #[inline]
    pub fn get_config_value(&self, key: &str) -> Option<&serde_json::Value> {
        self.config.get(key)
    }

    /// Tries to extract a value out of `secrets.json`.
    #[inline]
    pub fn get_secrets_value(&self, key: &str) -> Option<&serde_json::Value> {
        self.secrets.get(key)
    }
}
