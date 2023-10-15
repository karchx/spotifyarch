use std::sync::Arc;

use crate::{auth::AuthConfig, event::ClientRequest, state::SharedState};
use anyhow::Result;
use librespot_core::session::Session;
use rspotify::prelude::BaseClient;

mod handlers;
mod spotify;

#[derive(Clone)]
pub struct Client {
    http: reqwest::Client,
    pub spotify: Arc<spotify::Spotify>,
    pub client_pub: flume::Sender<ClientRequest>,
}

impl Client {
    pub fn new(
        session: Session,
        auth_config: AuthConfig,
        client_id: String,
        client_pub: flume::Sender<ClientRequest>,
    ) -> Self {
        Self {
            http: reqwest::Client::new(),
            spotify: Arc::new(spotify::Spotify::new(session, auth_config, client_id)),
            client_pub,
        }
    }

    pub async fn new_session(&self, _state: &SharedState) -> Result<()> {
        let session = crate::auth::new_session(&self.spotify.auth_config, false).await?;
        *self.spotify.session.lock().await = Some(session);
        tracing::info!("Used a new session for Spotify client.");

        Ok(())
    }

    /// initializes the authentication token inside the Spotify client
    pub async fn init_token(&self) -> Result<()> {
        self.spotify.refresh_token().await?;
        Ok(())
    }
}
