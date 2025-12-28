/// Smaller utilities for Fercord.
pub struct Utils;

impl Utils {
    /// Gets the path to the config directory.
    ///
    /// - Linux: `~/.config/fercord`
    /// - Windows: `[HOME]\fercord`
    pub fn get_config_dir_path() -> std::path::PathBuf {
        let home_dir = std::env::home_dir().expect("[ERROR] Unable to get home path!");
        let home_dir = home_dir.display();

        #[cfg(target_os = "linux")]
        return std::path::Path::new(&format!("{home_dir}/.config/fercord")).to_path_buf();

        #[cfg(target_os = "windows")]
        std::path::Path::new(&format!("{home_dir}\\fercord")).to_path_buf()
    }
}
