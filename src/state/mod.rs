mod model;
mod data;

use std::sync::RwLock;

pub use model::*;
pub use data::*;

use crate::config;
use anyhow::Result;

pub type SharedState = std::sync::Arc<State>;

pub struct State {
    pub app_config: config::AppConfig,

    pub cache_folder: std::path::PathBuf,
    pub data: RwLock<AppData>,
}

impl State {
    pub fn new(config_folder: &std::path::Path, cache_folder: &std::path::Path) -> Result<Self> {
        let state = Self {
            app_config: config::AppConfig::new(config_folder)?,
            cache_folder: cache_folder.to_path_buf(),
            data: RwLock::new(AppData::default()),
        };

        Ok(state)
    }
}
