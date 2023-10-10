mod auth;
mod client;
mod state;
mod token;
mod event;
mod config;

use anyhow::Result;

async fn start_app(state: state::SharedState) -> Result<()> {
    //let auth_config = auth::AuthConfig;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
     Ok(())
}
