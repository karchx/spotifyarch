extern crate dirs;
extern crate rpassword;

use std::fs;

use anyhow::Result;

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
        println!("ID: {}", client_id);
        println!("SECRET: {}", client_secret);
    }
    Ok(())
}

fn main() {
    init_spoterm_config_if_needed().unwrap();
}
