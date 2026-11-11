use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    pub global: GlobalConfig,
    pub plugins: PluginsConfig,
}

// Global configuration
#[derive(Debug, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct GlobalConfig {
    pub enabled: bool,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}

// Plugins configuration
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(default, deny_unknown_fields)]
pub struct PluginsConfig {
    pub url: Option<String>,
    pub email: Option<String>,
}

// Default config
impl Default for Config {
    fn default() -> Self {
        Self {
            global: GlobalConfig::default(),
            plugins: PluginsConfig::default(),
        }
    }
}

impl Config {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();

        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config: {}", path.display()))?;

        let config: Self = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config: {}", path.display()))?;

        Ok(config)
    }
}
