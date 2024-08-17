use std::sync::Arc;

use crate::{auth::AuthConfig, event::ClientRequest, state::*};
use anyhow::Result;
use librespot_core::session::Session;
use rspotify::{
    http::Query,
    prelude::{BaseClient, OAuthClient},
};

mod handlers;
mod spotify;

pub use handlers::*;

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

    pub async fn initialize_playback(&self, state:&SharedState) -> Result<()> {
        #[cfg(feature = "streaming")]
        if state.is_streaming_enabled() {
            self.new_streaming_connection(state).await;
        }

        self.retrieve_current_playback(state, false).await?;

        if state.player.read().playback.is_none() {
            tracing::info!("No playback found, trying to connecto to an available device...");

            // handle `connect_device` task separately as we don't want  to block here
            tokio::task::spawn({
                let client = self.clone();
                let state = state.clone();
                async move {
                    client.connect_device(&state).await;
                }
            });
        }

        Ok(())
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

    pub async fn handle_request(&self, state: &SharedState, request: ClientRequest) -> Result<()> {
        match request {
            ClientRequest::GetUserPlaylists => {
                let playlists = self.current_user_playlists().await?;
                state.data.write().user_data.playlists = playlists;
            }
        }

        Ok(())
    }

    /// gets all playlist of the current user
    pub async fn current_user_playlists(&self) -> Result<Vec<Playlist>> {
        let first_page = self
            .spotify
            .current_user_playlists_manual(Some(50), None)
            .await?;

        let playlists = self.all_paging_items(first_page, &Query::new()).await?;
        Ok(playlists.into_iter().map(|p| p.into()).collect())
    }

    async fn all_paging_items<T>(
        &self,
        first_page: rspotify_model::Page<T>,
        payload: &Query<'_>,
    ) -> Result<Vec<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut items = first_page.items;
        let mut maybe_next = first_page.next;

        while let Some(url) = maybe_next {
            let mut next_page = self
                .internal_call::<rspotify_model::Page<T>>(&url, payload)
                .await?;
            items.append(&mut next_page.items);
            maybe_next = next_page.next;
        }
        Ok(items)
    }

    async fn internal_call<T>(&self, url: &str, payload: &Query<'_>) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let access_token = self.spotify.access_token().await?;
        Ok(self
            .http
            .get(url)
            .query(payload)
            .header(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {access_token}"),
            )
            .send()
            .await?
            .json::<T>()
            .await?)
    }
}
