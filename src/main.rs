mod classifier;
mod daemon;
mod registry;

use anyhow::Result;
use daemon::ClipboardDaemon;

fn main() -> Result<()> {
    // Init logging
    tracing_subscriber::fmt::init();

    tracing::info!("Clipflow listening for clipboard changes ...");

    ClipboardDaemon::new()?.monitor_clipboard_events()?;

    Ok(())
}
