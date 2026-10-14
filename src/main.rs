mod daemon;

use anyhow::Result;
use daemon::ClipboardDaemon;

fn main() -> Result<()> {
    // Init logging
    tracing_subscriber::fmt::init();
    let mut clipboard_daemon = ClipboardDaemon::new()?;

    clipboard_daemon.monitor_clipboard_events()?;
    
    tracing::info!("Clipflow listening for clipboard changes ...");

    Ok(())
}
