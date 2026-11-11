use crate::classifier::ClassifiedEntry;
use crate::classifier::ClipboardEntryType::{Email, Unknown, Url};
use anyhow::Result;
use std::collections::HashMap;

pub struct Registry {
    _plugins_registry: HashMap<String, String>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            _plugins_registry: HashMap::new(),
            // We need to initialize the registry with existing plugins
            // Only those enabled, and 1 plugin enabled per type
        }
    }
    pub fn actions_for(&self, entry: &ClassifiedEntry) -> Result<()> {
        match entry.kind {
            // We call the right enabled plugin depending on the entry type
            Url => {}
            Email => {}
            Unknown => {} // Do nothing
        }

        Ok(())
    }
}
