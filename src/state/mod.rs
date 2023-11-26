mod data;
mod model;
mod ui;

use anyhow::Result;
pub use data::*;
pub use model::*;
pub use ui::*;
use parking_lot::{RwLock, Mutex};

use crate::config;

pub type SharedState = std::sync::Arc<State>;

#[derive(Debug)]
pub struct Configs {
    pub app_config: config::AppConfig,
    pub cache_folder: std::path::PathBuf,
}

impl Configs {
    pub fn new(config_folder: &std::path::Path, cache_folder: &std::path::Path) -> Result<Self> {
        Ok(Self {
            app_config: config::AppConfig::new(config_folder)?,
            cache_folder: cache_folder.to_path_buf(),
        })
    }
}

pub struct State {
    pub configs: Configs,
    pub data: RwLock<AppData>,
    pub ui: Mutex<UIState>,
}

impl State {
    pub fn new(configs: Configs) -> Self {
        let mut ui = UIState::default();
        Self {
            configs,
            data: RwLock::new(AppData::default()),
            ui: Mutex::new(ui),
        }
    }
}
