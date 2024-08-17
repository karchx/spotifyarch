mod data;
mod model;
mod ui;

use anyhow::Result;
pub use data::*;
pub use model::*;
use parking_lot::{Mutex, RwLock};
pub use ui::*;

use crate::config;

pub type SharedState = std::sync::Arc<State>;

/// Aplication state
pub struct State {
    pub player: RwLock<PlayerState>,
    pub data: RwLock<AppData>,
    pub ui: Mutex<UIState>,

    pub is_daemon: bool,
}

impl State {
    pub fn new(is_daemon: bool) -> Self {
        let mut ui = UIState::default();
        let configs = config::get_config();

        let app_data = AppData::new(&config.cache_folder);

        Self {
            ui: Mutex::new(ui),
            player: RwLock::new(PlayerState::default()), // TODO: player state implementation
            data: RwLock::new(app_data),
            is_daemon,
        }
    }

    #[cfg(feature = "streaming")]
    pub fn is_streaming_enabled(&self) -> bool {
        let configs = config::get_config();
        configs.app_config.enable_streaming == config::StreamingType::Always
            || (config.app_config.enable_streaming == config::StreamingType::DaemonOnly
                && self.is_daemon)
    }
}
