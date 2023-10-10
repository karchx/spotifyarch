
use anyhow::{anyhow, Result};
use librespot_core::session::Session;
use maybe_async::maybe_async;
use rspotify::{clients::{OAuthClient, BaseClient}, Credentials, OAuth, Config, sync::Mutex, Token, http::HttpClient, ClientResult};

use crate::auth::{AuthConfig, token};

use std::{sync::Arc, fmt};

#[derive(Clone, Default)]
/// A Spotify Client to interact with Spotify API server
pub struct Spotify {
    pub auth_config: AuthConfig,
    pub creds: Credentials,
    pub oauth: OAuth,
    pub config: Config,
    pub token: Arc<Mutex<Option<Token>>>,
    pub client_id: String,
    pub http: HttpClient,
    pub session: Arc<tokio::sync::Mutex<Option<Session>>>,
}

impl fmt::Debug for Spotify {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Spotify")
            .field("creds", &self.creds)
            .field("oauth", &self.oauth)
            .field("config", &self.config)
            .field("token", &self.token)
            .field("client_id", &self.client_id)
            .finish()
    }
}

impl Spotify {
    pub fn new(session: Session, auth_config: AuthConfig, client_id: String) -> Spotify {
        Self {
            creds: Credentials::default(),
            oauth: OAuth::default(),
            config: Config {
                token_refreshing: true,
                ..Default::default()
            },
            token: Arc::new(Mutex::new(None)),
            http: HttpClient::default(),
            session: Arc::new(tokio::sync::Mutex::new(Some(session))),
            auth_config,
            client_id,
        }
    }


    pub async fn session(&self) -> Session {
        self.session
            .lock()
            .await
            .clone()
            .expect("Spotify client's session should not be empty")
    }
}

#[maybe_async]
impl BaseClient for Spotify {
    fn get_http(&self) -> &HttpClient {
        &self.http
    }

    fn get_token(&self) -> Arc<Mutex<Option<Token>>> {
        Arc::clone(&self.token)
    }

    fn get_creds(&self) -> &Credentials {
        &self.creds
    }

    fn get_config(&self) -> &Config {
        &self.config
    }

    async fn refetch_token(&self) -> ClientResult<Option<Token>> {
        let session = self.session().await;
        let old_token = self.token.lock().await.unwrap().clone();

        if session.is_invalid() {
            tracing::error!("Failed to get a new token: invalid session");
            return Ok(old_token);
        }

        match token::get_token(&session, &self.client_id).await {}
    }
}

/// Implement `OAuthClient` trait for `Spotify` struct
/// to allow calling methods that get/modify user's data such as
/// `current_user_playlists`, `playlist_add_items`, etc.
///
/// Because the `Spotify` client interacts with Spotify APIs
/// using an access token that is manually retrieved by
/// the `librespot::get_token` function, implementing
/// `OAuthClient::get_oauth` and `OAuthClient::request_token` is unnecessary
#[maybe_async]
impl OAuthClient for Spotify {
    fn get_oauth(&self) -> &OAuth {
        panic!("`OAuthClient::get_oauth` should never be calle")
    }

    async fn request_token(&self, _code: &str) -> ClientResult<()> {
        panic!("`OAuthClient::request_token` should never be calle")
    }
}


