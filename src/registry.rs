use crate::classifier::ClassifiedEntry;
use anyhow::Result;
use std::collections::HashMap;

pub struct Registry {
    plugins_registry: HashMap<String, String>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            plugins_registry: HashMap::new(),
        }
    }
    pub fn actions_for(&self, entry: &ClassifiedEntry) -> Result<()> {
        println!("{:?}", entry.kind);
        Ok(())
    }
}
