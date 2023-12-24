mod auth;
mod cli;
mod client;
mod config;
mod event;
mod state;
mod token;
mod ui;
mod utils;

use anyhow::{Context, Result};

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
    let (client_pub, client_sub) = flume::unbounded::<event::ClientRequest>();

    let auth_config = auth::AuthConfig::new(&state)?;
    let session = auth::new_session(&auth_config, true).await?;
    // create a spotify API client
    let client = client::Client::new(
        session,
        auth_config,
        state.configs.app_config.client_id.clone(),
        client_pub.clone(),
    );
    client.init_token().await?;

    init_spotify(&client_pub, &client, &state)
        .await
        .context("Failed to initialize the Spotify data")?;

    let mut tasks = Vec::new();

    tasks.push(tokio::task::spawn({
        let state = state.clone();
        let client = client.clone();
        async move {
            client::start_client_handler(state, client, client_sub).await;
        }
    }));

    // application UI task
    tokio::task::spawn_blocking({
        let state = state.clone();
        move || ui::run(state)
    });

    for task in tasks {
        task.await?;
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = cli::init_cli()?.get_matches();

    let config_folder: std::path::PathBuf = config::get_config_folder_path()?;
    let cache_folder: std::path::PathBuf = config::get_cache_folder_path()?;

    // initialize the application configs
    let configs = state::Configs::new(&config_folder, &cache_folder)?;

    match args.subcommand() {
        None => {
            // log the application's configurations
            tracing::info!("Configurations: {:?}", configs);
            let state = std::sync::Arc::new(state::State::new(configs));
            start_app(state);
        }
        Some(_) => {}
    }
    Ok(())
}
