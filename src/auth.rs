use anyhow::Result;
use librespot_core::{
    cache::Cache,
    config::SessionConfig,
};

use crate::state::SharedState;

#[derive(Clone)]
pub struct AuthConfig {
    pub cache: Cache,
    pub session_config: SessionConfig
}

impl Default for AuthConfig {
    fn default() -> Self {
        AuthConfig {
            cache: Cache::new(None::<String>,  None, None, None).unwrap(),
            session_config: SessionConfig::default(),
        }
    }
}

//impl AuthConfig {
//    pub fn new(state: &SharedState) -> Result<AuthConfig> {
//        let audio_cache_folder
//    }
//}
