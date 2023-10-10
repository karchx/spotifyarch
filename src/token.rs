
use std::collections::HashSet;

use anyhow::Result;
use chrono::{Duration, Utc};
use librespot_core::{keymaster, session::Session};
use rspotify::Token;

// https://developer.spotify.com/documentation/general/guides/scopes/
pub const SCOPES: [&str; 2] = [
    // playlists
    "playlist-read-private",
    "streaming",
];

const TIMEOUT_IN_SECS: u64 = 5;

pub async fn get_token(session: &Session, client_id: &str) -> Result<Token> {
    tracing::info!("Getting new authentication token...");

    let scopes = SCOPES.join(",");

    let fut = keymaster::get_token(session, client_id, &scopes);
    let token = 
        match tokio::time::timeout(std::time::Duration::from_secs(TIMEOUT_IN_SECS), fut).await {
            Ok(Ok(token)) => token,
            Ok(Err(err)) => anyhow::bail!("failed to get the token: {:?}", err),
            Err(_) => {
                if !session.is_invalid() {
                    session.shutdown();
                }
                anyhow::bail!("timeout when getting the token");
            }
        };

    let expires_in = Duration::from_std(std::time::Duration::from_secs(token.expires_in as u64))?;
    let expires_at = Utc::now() + expires_in;

    let token = Token {
        access_token: token.access_token,
        expires_in,
        expires_at: Some(expires_at),
        scopes: HashSet::new(),
        refresh_token: None,
    };

    tracing::info!("Got new token: {token:?}");

    Ok(token)
}
