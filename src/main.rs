mod auth;
mod client;
mod config;
mod event;
mod state;
mod token;

use anyhow::Result;


#[tokio::main]
async fn start_app(state: state::SharedState) -> Result<()> {
    let auth_config = auth::AuthConfig::new(&state)?;
    let _session = auth::new_session(&auth_config, true).await?;
    Ok(())
}

fn main() -> Result<()> {
    let config_folder: std::path::PathBuf = config::get_config_folder_path()?;
    let cache_folder: std::path::PathBuf = config::get_cache_folder_path()?;

    let state = std::sync::Arc::new(state::State::new(&config_folder, &cache_folder)?);
    start_app(state);
    Ok(())
}
