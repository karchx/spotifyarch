mod auth;
mod client;
mod state;
mod token;

extern crate dirs;
extern crate rpassword;
extern crate rspotify;
extern crate toml;

use std::fs;

use anyhow::Result;
use spotifyarch::config::UserConfig;


// https://developer.spotify.com/documentation/general/guides/scopes/
pub const SCOPES: [&str; 1] = [
    // playlists
    "playlist-read-private",
];

fn init_spoterm_config_if_needed() -> Result<()> {
    let config_dir = dirs::home_dir()
        .expect("can not find home directory")
        .join(".spotifyarch");
    //create a config dir ~/.spotifyarch/ if needed
    if !config_dir.exists() {
        fs::create_dir_all(config_dir.clone())?;
    }
    //create a config file ~/.spotifyarch/config.toml if need
    let config = config_dir.join("config.toml");
    if !config.exists() {
        println!("config.toml not found and input your <CLIENT_ID> and <CLIENT_SECRET>");
        let client_id = rpassword::read_password_from_tty(Some("Client ID: "))?;
        let client_secret = rpassword::read_password_from_tty(Some("Client Secret: "))?;
        let user_config = UserConfig::new()
            .client_id(client_id)
            .client_secret(client_secret);

        fs::write(config.as_path(), toml::to_string(&user_config)?)?;
    }
    Ok(())
}

fn get_spotify_client_id_and_secret() -> Result<(String, String), Box<dyn std::error::Error>> {
    let config = dirs::home_dir()
        .expect("can not find home directory")
        .join(".spotifyarch")
        .join("config.toml");
    let config_content = fs::read_to_string(config.to_str().expect("can not read config file"))?;
    let user_config: UserConfig = toml::from_str(&config_content)?;

    Ok((
        user_config.profile.client_id,
        user_config.profile.client_secret,
    ))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    //init_spoterm_config_if_needed().unwrap();

    //let (client_id, client_secret) = get_spotify_client_id_and_secret().unwrap();

    //let spotifyarch_cache = dirs::home_dir()
    //    .expect("can not find home directory")
    //    .join(".spotifyarch")
    //    .join(".spotify_token_cache.json");

    //let oauth = OAuth::new("http://localhost:8888/callback", &SCOPES.join(" "));
    //let creds = Credentials::new(&client_id, &client_secret);
    //let mut spotify = AuthCodeSpotify::new(creds, oauth);
   // let mut oauth = rspotify::oauth2::SpotifyOAuth::default()
   //     .scope(&SCOPES.join(" "))
   //     .client_id(&client_id)
   //     .client_secret(&client_secret)
   //     .redirect_uri()
   //     .cache_path(spotifyarch_cache)
   //     .build();

    //let token_info = rspotify::util::get_token(&mut oauth).await.unwrap();
    let token_info = "working...";
    println!("{:?}", token_info);

    Ok(())
}
