use anyhow::{anyhow, Result};
use config_parser2::*;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

const DEFAULT_CONFIG_FOLDER: &str = ".config/spotifyarch";
const DEFAULT_CACHE_FOLDER: &str = ".cache/spotifyarch";
const APP_CONFIG_FILE: &str = "config.toml";

#[derive(Debug, Deserialize, Serialize, ConfigParse)]
pub struct AppConfig {
    pub client_id: String,
    pub client_port: u16,
    pub default_device: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            client_id: "7262364c3d2a4642993653f18072fb9b".to_string(),
            client_port: 8080,
            default_device: "spotifyarch".to_string(),
        }
    }
}

impl AppConfig {
    pub fn new(path: &Path) -> Result<Self> {
        let mut config = Self::default();
        //if !config.parse_config
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
        toml::to_string_pretty(&self)
            .map_err(From::from)
            .and_then(|content| {
                std::fs::write(path.join(APP_CONFIG_FILE), content).map_err(From::from)
            })
    }
}

pub fn get_config_folder_path() -> Result<PathBuf> {
    match dirs::home_dir() {
        Some(home) => Ok(home.join(DEFAULT_CONFIG_FOLDER)),
        None => Err(anyhow!("cannot find the $HOME folder")),
    }
}

pub fn get_cache_folder_next() -> Result<PathBuf> {
    match  dirs::home_dir() {
        Some(home) => Ok(home.join(DEFAULT_CACHE_FOLDER)),
        None => Err(anyhow!("cannon find the $HOME folder"))
    }
}
