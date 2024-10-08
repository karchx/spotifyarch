const DEFAULT_CONFIG_FOLDER: &str = ".config/spotifyarch";
const DEFAULT_CACHE_FOLDER: &str = ".cache/spotifyarch";
const APP_CONFIG_FILE: &str = "config.toml";

use anyhow::{anyhow, Result};
use config_parser2::*;
use librespot_core::config::SessionConfig;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::OnceLock,
};

static CONFIGS: OnceLock<Configs> = OnceLock::new();

#[derive(Debug)]
pub struct Configs {
    pub app_config: AppConfig,
    pub cache_folder: std::path::PathBuf,
}

impl Configs {
    pub fn new(config_folder: &std::path::Path, cache_folder: &std::path::Path) -> Result<Self> {
        Ok(Self {
            app_config: AppConfig::new(config_folder)?,
            cache_folder: cache_folder.to_path_buf(),
        })
    }
}

#[derive(Debug, Deserialize, Serialize, ConfigParse)]
pub struct AppConfig {
    pub client_id: String,

    pub client_port: u16,
    pub default_device: String,
    pub ap_port: Option<u16>,

    pub app_refresh_duration_in_ms: u64,

    pub playback_window_width: usize,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum BorderType {
    Hidden,
    Plain,
    Rounded,
    Double,
    Thick,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            client_id: "7262364c3d2a4642993653f18072fb9b".to_string(),
            client_port: 8080,
            default_device: "spotifyarch".to_string(),
            ap_port: None,
            app_refresh_duration_in_ms: 32,
            playback_window_width: 6,
        }
    }
}

impl AppConfig {
    pub fn new(path: &Path) -> Result<Self> {
        let mut config = Self::default();
        if !config.parse_config_file(path)? {
            config.write_config_file(path)?
        }

        Ok(config)
    }

    fn parse_config_file(&mut self, path: &Path) -> Result<bool> {
        let file_path = path.join(APP_CONFIG_FILE);
        match std::fs::read_to_string(file_path) {
            Ok(content) => self
                .parse(toml::from_str::<toml::Value>(&content)?)
                .map(|_| true),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
            Err(error) => Err(error.into()),
        }
    }

    fn write_config_file(&self, path: &Path) -> Result<()> {
        if !std::fs::metadata(&path).is_ok() {
            if let Err(err) = std::fs::create_dir_all(&path) {
                tracing::error!("Failed to create directory: {}", err);
            } else {
                tracing::info!("Directory created successfully");
            }
        }

        toml::to_string_pretty(&self)
            .map_err(From::from)
            .and_then(|content| {
                std::fs::write(path.join(APP_CONFIG_FILE), content).map_err(From::from)
            })
    }

    pub fn session_config(&self) -> SessionConfig {
        SessionConfig {
            proxy: None,
            ap_port: self.ap_port,
            ..Default::default()
        }
    }
}

pub fn get_config_folder_path() -> Result<PathBuf> {
    match dirs::home_dir() {
        Some(home) => Ok(home.join(DEFAULT_CONFIG_FOLDER)),
        None => Err(anyhow!("cannot find the $HOME folder")),
    }
}

pub fn get_cache_folder_path() -> Result<PathBuf> {
    match dirs::home_dir() {
        Some(home) => Ok(home.join(DEFAULT_CACHE_FOLDER)),
        None => Err(anyhow!("cannon find the $HOME folder")),
    }
}

#[inline(always)]
pub fn get_config() -> &'static Configs {
    CONFIGS.get().expect("configs is already initialized")
}

pub fn set_config(configs: Configs) {
    CONFIGS
        .set(configs)
        .expect("configs should be initialized only once")
}
