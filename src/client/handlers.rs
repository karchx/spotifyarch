use tracing::Instrument;

use crate::{event::ClientRequest, state::*};

pub async fn start_client_request(
    state: SharedState,
    client: super::Client,
    client_sub: flume::Receiver<ClientRequest>,
) {
    while let Ok(request) = client_sub.recv_async().await {
        if client.spotify.session().await.is_invalid() {
            tracing::info!("Spotify client's session is invalid, re-creating a new session...");
            if let Err(err) = client.new_session(&state).await {
                tracing::error!("Failed to create a new session: {err:#}");
                continue;
            }
        }

        let state = state.clone();
        let client = client.clone();
        let span = tracing::info_span!("client_request", request = ?request);

        tokio::task::spawn(
            async move {
                if let Err(err) = client.handle_request(&state, request).await {
                    tracing::error!("Failed to handle client request {err:#}");
                }
            }
            .instrument(span),
        );
    }
}
