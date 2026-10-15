use anyhow::Result;

use crate::daemon::ClipboardEntry;

pub struct ClipboardEntryType {}
pub struct Classifier {}

impl Classifier {
    pub fn _new() -> Result<Self> {
        Ok(Self {})
    }

    pub fn infer_entry_type(&self, entry: ClipboardEntry) -> Result<ClipboardEntryType> {
        println!("{:?}", entry);
        Ok(ClipboardEntryType {})
    }
}
