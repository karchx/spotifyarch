use std::sync::Arc;

use librespot_core::session::Session;
use anyhow::Result;
use crate::{event::ClientRequest, auth::AuthConfig, state::SharedState};

mod handlers;
mod spotify;

#[derive(Clone)]
pub struct Client {
    http:  reqwest::Client,
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

    pub async fn new_session(&self, state: &SharedState) -> Result<()> {
        let session = crate::auth::new_session();
    }
}
