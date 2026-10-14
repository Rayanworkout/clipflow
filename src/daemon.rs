use anyhow::{Context, Result, anyhow};
use arboard::{Clipboard, Error as ClipboardError};
use serde::{Deserialize, Serialize};
use std::thread::sleep;
use std::time::Duration;

const CLIPBOARD_REFRESH_RATE_MS: u64 = 800;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum ClipboardEntry {
    Text(String),
}

pub struct ClipboardDaemon {
    clipboard: Clipboard,
}

impl ClipboardDaemon {
    pub fn new() -> Result<Self> {
        // Instanciate a clipboard object that will be used to access
        // or update the system clipboard.

        Ok(Self {
            clipboard: Clipboard::new()
                .context("Could not create a clipboard_daemon instance")?
                .into(),
        })
    }

    pub fn monitor_clipboard_events(&mut self) -> Result<()> {
        loop {
            match self.read_clipboard_entry() {
                Ok(Some(entry)) => {
                    println!("New clipboard entry: {:?}", entry);
                }
                Ok(None) => {}
                Err(error) => {
                    tracing::error!("Error getting clipboard content: {}", error);
                }
            }

            sleep(Duration::from_millis(CLIPBOARD_REFRESH_RATE_MS));
        }
    }

    fn read_clipboard_entry(&mut self) -> Result<Option<ClipboardEntry>> {
        match self.clipboard.get_text() {
            Ok(content) => {
                if !content.trim().is_empty() {
                    return Ok(Some(ClipboardEntry::Text(content)));
                }
            }
            Err(ClipboardError::ContentNotAvailable) => {}
            Err(text_error) => {
                return Err(anyhow!(
                    "Could not get clipboard text content: {text_error}"
                ));
            }
        }

        Ok(None)
    }
}
