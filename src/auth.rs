use anyhow::{anyhow, Result};
use librespot_core::{
    authentication::Credentials,
    cache::Cache,
    config::SessionConfig,
    session::{Session, SessionError},
};
use std::io::Write;

use crate::state::SharedState;

#[derive(Clone)]
pub struct AuthConfig {
    pub cache: Cache,
    pub session_config: SessionConfig,
}

impl Default for AuthConfig {
    fn default() -> Self {
        AuthConfig {
            cache: Cache::new(None::<String>, None, None, None).unwrap(),
            session_config: SessionConfig::default(),
        }
    }
}

impl AuthConfig {
    pub fn new(state: &SharedState) -> Result<AuthConfig> {
        let cache = Cache::new(Some(state.configs.cache_folder.clone()), None, None, None)?;

        let auth_config = AuthConfig {
            cache,
            session_config: state.configs.app_config.session_config(),
        };
        Ok(auth_config)
    }
}

fn read_user_auth_detail(user: Option<String>) -> Result<(String, String)> {
    let mut username = String::new();
    let mut stdout = std::io::stdout();
    match user {
        None => write!(stdout, "Username: ")?,
        Some(ref u) => write!(stdout, "Username (default: {u}): ")?,
    }
    stdout.flush()?;
    std::io::stdin().read_line(&mut username)?;
    username = username.trim_end().to_string();
    if username.is_empty() {
        username = user.unwrap_or_default();
    }
    let password = rpassword::prompt_password(format!("Password for {username}: "))?;
    Ok((username, password))
}

pub async fn new_session_with_creds(auth_config: &AuthConfig) -> Result<Session> {
    tracing::info!("Creating a new session with new authenticate credentials");

    let mut user: Option<String> = None;

    for i in 0..3 {
        let (username, password) = read_user_auth_detail(user)?;
        user = Some(username.clone());
        match Session::connect(
            auth_config.session_config.clone(),
            Credentials::with_password(username, password),
            Some(auth_config.cache.clone()),
            true,
        )
        .await
        {
            Ok((session, _)) => {
                println!("Successfully authenticated as {}", user.unwrap_or_default());
                return Ok(session);
            }
            Err(err) => {
                eprintln!("Failed to authtenticated, {} tries left", 2 - i);
                tracing::warn!("Failed to authenticate {err:#}");
            }
        }
    }
    Err(anyhow!("authentication failed"))
}

pub async fn new_session(auth_config: &AuthConfig, reauth: bool) -> Result<Session> {
    match auth_config.cache.credentials() {
        None => {
            let msg = "No cached credentials found, please authenticate";
            if reauth {
                eprintln!("{msg}");
                new_session_with_creds(auth_config).await
            } else {
                anyhow::bail!(msg);
            }
        }
        Some(creds) => {
            match Session::connect(
                auth_config.session_config.clone(),
                creds,
                Some(auth_config.cache.clone()),
                true,
            )
            .await
            {
                Ok((session, _)) => {
                    tracing::info!(
                        "Successfully used the cached credentials to create a new session!"
                    );
                    Ok(session)
                }
                Err(err) => match err {
                    SessionError::AuthenticationError(err) => {
                        let msg =
                            format!("Failed to authenticate using cached credentials: {err:#}");
                        if reauth {
                            eprintln!("{msg}");
                            new_session_with_creds(auth_config).await
                        } else {
                            anyhow::bail!(msg)
                        }
                    }
                    SessionError::IoError(err) => {
                        anyhow::bail!("{err}\nPlease check your internet connection.");
                    }
                },
            }
        }
    }
}
