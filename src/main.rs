mod auth;
mod client;
mod config;
mod event;
mod state;
mod token;

use anyhow::Result;

async fn init_spotify(
    client_pub: &flume::Sender<event::ClientRequest>,
    _client: &client::Client,
    _state: &state::SharedState,
) -> Result<()> {
    client_pub.send(event::ClientRequest::GetUserPlaylists)?;

    Ok(())
}

#[tokio::main]
async fn start_app(state: state::SharedState) -> Result<()> {
    // client channels
    let (client_pub, _client_sub) = flume::unbounded::<event::ClientRequest>();

    let auth_config = auth::AuthConfig::new(&state)?;
    let session = auth::new_session(&auth_config, true).await?;
    // create a spotify API client
    let client = client::Client::new(
        session,
        auth_config,
        state.app_config.client_id.clone(),
        client_pub.clone(),
    );
    client.init_token().await?;

    init_spotify(&client_pub, &client, &state)
        .await
        .expect("Failed to initialize the Spotify data");

    Ok(())
}

fn main() -> Result<()> {
    let config_folder: std::path::PathBuf = config::get_config_folder_path()?;
    let cache_folder: std::path::PathBuf = config::get_cache_folder_path()?;

    let state = std::sync::Arc::new(state::State::new(&config_folder, &cache_folder)?);

    if let Err(err) = start_app(state) {
        tracing::error!("Encountered an error when running the application: {err:#}");
    }
    Ok(())
}
