// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! Configuration management for Marshall browser

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;

mod defaults;

pub use defaults::*;

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub privacy: PrivacyConfig,
    pub adblock: AdblockConfig,
    pub appearance: AppearanceConfig,
    pub network: NetworkConfig,
    pub keybindings: KeybindingsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub homepage: String,
    pub search_engine: String,
    pub download_dir: String,
    pub restore_session: bool,
    pub enable_javascript: bool,
    pub enable_images: bool,
    pub enable_webgl: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyConfig {
    pub strict_mode: bool,
    pub block_trackers: bool,
    pub block_fingerprinting: bool,
    pub block_third_party_cookies: bool,
    pub clear_on_exit: bool,
    pub do_not_track: bool,
    pub referrer_policy: String,
    pub user_agent: Option<String>,
    pub spoof_timezone: bool,
    pub spoof_language: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdblockConfig {
    pub enabled: bool,
    pub filter_lists: Vec<String>,
    pub custom_rules: Vec<String>,
    pub whitelist: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceConfig {
    pub theme: String,
    pub dark_mode: bool,
    pub show_toolbar: bool,
    pub show_bookmarks_bar: bool,
    pub show_status_bar: bool,
    pub font_family: String,
    pub font_size: u32,
    pub zoom_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub proxy: Option<ProxyConfig>,
    pub tor_enabled: bool,
    pub dns_over_https: bool,
    pub doh_server: String,
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    pub protocol: String,
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeybindingsConfig {
    pub new_tab: String,
    pub close_tab: String,
    pub reload: String,
    pub back: String,
    pub forward: String,
    pub find: String,
    pub zoom_in: String,
    pub zoom_out: String,
    pub zoom_reset: String,
    pub developer_tools: String,
    pub private_window: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            general: GeneralConfig {
                homepage: "about:blank".to_string(),
                search_engine: "https://duckduckgo.com/?q=".to_string(),
                download_dir: dirs::download_dir()
                    .unwrap_or_else(|| PathBuf::from("~/Downloads"))
                    .to_string_lossy()
                    .to_string(),
                restore_session: false,
                enable_javascript: true,
                enable_images: true,
                enable_webgl: false, // Disabled for privacy
            },
            privacy: PrivacyConfig {
                strict_mode: true,
                block_trackers: true,
                block_fingerprinting: true,
                block_third_party_cookies: true,
                clear_on_exit: true,
                do_not_track: true,
                referrer_policy: "strict-origin-when-cross-origin".to_string(),
                user_agent: None, // Use randomized UA
                spoof_timezone: true,
                spoof_language: false,
            },
            adblock: AdblockConfig {
                enabled: true,
                filter_lists: vec![
                    "https://easylist.to/easylist/easylist.txt".to_string(),
                    "https://easylist.to/easylist/easyprivacy.txt".to_string(),
                    "https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts".to_string(),
                ],
                custom_rules: vec![],
                whitelist: vec![],
            },
            appearance: AppearanceConfig {
                theme: "nullsec-dark".to_string(),
                dark_mode: true,
                show_toolbar: true,
                show_bookmarks_bar: true,
                show_status_bar: true,
                font_family: "JetBrains Mono".to_string(),
                font_size: 14,
                zoom_level: 1.0,
            },
            network: NetworkConfig {
                proxy: None,
                tor_enabled: false,
                dns_over_https: true,
                doh_server: "https://cloudflare-dns.com/dns-query".to_string(),
                timeout_seconds: 30,
            },
            keybindings: KeybindingsConfig {
                new_tab: "<Ctrl>t".to_string(),
                close_tab: "<Ctrl>w".to_string(),
                reload: "<Ctrl>r".to_string(),
                back: "<Alt>Left".to_string(),
                forward: "<Alt>Right".to_string(),
                find: "<Ctrl>f".to_string(),
                zoom_in: "<Ctrl>plus".to_string(),
                zoom_out: "<Ctrl>minus".to_string(),
                zoom_reset: "<Ctrl>0".to_string(),
                developer_tools: "F12".to_string(),
                private_window: "<Ctrl><Shift>p".to_string(),
            },
        }
    }
}

impl Config {
    /// Load configuration from file
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Self::config_path()?;
        
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            // Create default config
            let config = Config::default();
            config.save()?;
            Ok(config)
        }
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::config_path()?;
        
        // Create parent directories if needed
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        fs::write(&config_path, content)?;
        Ok(())
    }

    /// Get config file path
    fn config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        if let Some(proj_dirs) = ProjectDirs::from("io", "nullsec", "marshall") {
            Ok(proj_dirs.config_dir().join("config.toml"))
        } else {
            // Fallback
            Ok(PathBuf::from("~/.config/marshall/config.toml"))
        }
    }

    /// Get data directory path
    pub fn data_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
        if let Some(proj_dirs) = ProjectDirs::from("io", "nullsec", "marshall") {
            Ok(proj_dirs.data_dir().to_path_buf())
        } else {
            Ok(PathBuf::from("~/.local/share/marshall"))
        }
    }

    /// Get cache directory path
    pub fn cache_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
        if let Some(proj_dirs) = ProjectDirs::from("io", "nullsec", "marshall") {
            Ok(proj_dirs.cache_dir().to_path_buf())
        } else {
            Ok(PathBuf::from("~/.cache/marshall"))
        }
    }
}

fn dirs_download_dir() -> Option<PathBuf> {
    std::env::var_os("HOME").map(|home| PathBuf::from(home).join("Downloads"))
}

mod dirs {
    use super::*;
    
    pub fn download_dir() -> Option<PathBuf> {
        std::env::var_os("XDG_DOWNLOAD_DIR")
            .map(PathBuf::from)
            .or_else(|| {
                std::env::var_os("HOME").map(|home| PathBuf::from(home).join("Downloads"))
            })
    }
}
