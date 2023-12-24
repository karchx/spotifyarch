pub fn init_cli() -> anyhow::Result<clap::Command> {
    let cmd = clap::Command::new("spotifyarch");

    Ok(cmd)
}
