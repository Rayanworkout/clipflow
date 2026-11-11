mod classifier;
mod config;
mod daemon;
mod registry;

use anyhow::Result;
use config::Config;
use daemon::ClipboardDaemon;

fn main() -> Result<()> {
    // Init logging
    tracing_subscriber::fmt::init();

    // Init the config file
    let config = Config::load("config.toml")?;

    println!("{:?}", config);

    tracing::info!("Clipflow listening for clipboard changes ...");

    ClipboardDaemon::new()?.monitor_clipboard_events()?;

    Ok(())
}
