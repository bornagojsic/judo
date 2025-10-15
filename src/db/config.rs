use crate::ui::theme::Theme;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

// Default variables
const DEFAULT_DB_NAME: &str = "dojo";
const DEFAULT_DB_FILE: &str = "judo.db";

/// Config file definition
#[derive(Deserialize, Serialize)]
pub struct Config {
    pub default: String,
    pub dbs: Vec<DBConfig>,
    pub theme: Option<Theme>,
}

/// Database configuration
#[derive(Deserialize, Serialize, Clone)]
pub struct DBConfig {
    pub name: String,
    pub connection_str: String,
}

impl Default for DBConfig {
    fn default() -> Self {
        // Use data directory to standardize storage
        let data_dir = dirs::data_dir().unwrap().join("judo");

        // Create directory
        std::fs::create_dir_all(&data_dir).unwrap();

        // Create path to db
        let path = data_dir.join(DEFAULT_DB_FILE);

        // Create connection string (only SQLite is admissible)
        let connection_str = format!("sqlite:{}", path.display());

        Self {
            name: DEFAULT_DB_NAME.to_string(),
            connection_str,
        }
    }
}

impl Default for Config {
    /// By default, the name is the default name with default config
    fn default() -> Self {
        Self {
            default: DEFAULT_DB_NAME.to_string(),
            dbs: vec![DBConfig::default()],
            theme: Some(Theme::default()),
        }
    }
}

impl Config {
    /// Write config struct to judo.toml file
    pub fn write(&self, config_path: &PathBuf) -> Result<()> {
        // Convert config to string to be written to config file
        let toml_content =
            toml::to_string_pretty(&self).with_context(|| "Failed to serialize judo.toml")?;

        // Write string to file
        fs::write(config_path, toml_content).with_context(|| {
            format!(
                "Failed to write yomo.toml file to {}",
                config_path.display()
            )
        })?;

        Ok(())
    }

    /// Read and serialize a judo.toml file
    pub fn read() -> Result<Self> {
        // Use config directory to standardize storage of config file
        let config_dir = dirs::config_dir().unwrap().join("judo");

        // Define the config file path
        let config_path = config_dir.join("judo.toml");

        // Create config if not existing
        if !config_dir.exists() | !config_path.exists() {
            // Create directory
            std::fs::create_dir_all(&config_dir)
                .with_context(|| "Failed to create config directory")?;

            // Create default config
            let config = Self::default();

            // Create config file
            config
                .write(&config_path)
                .with_context(|| "Failed to create config file")?;

            // Create default config
            return Ok(Self::default());
        }

        // Serialize judo.toml into YomoProject struct
        let judo_config: Config = toml::from_str(
            &fs::read_to_string(config_path).with_context(|| "Failed to read into string")?,
        )
        .with_context(|| "Failed to serialize into struct")?;

        Ok(judo_config)
    }

    /// Get config of default database
    pub fn get_default(&self) -> Result<DBConfig> {
        let matching_dbs: Vec<_> = self
            .dbs
            .iter()
            .filter(|db| db.name == self.default)
            .collect();

        match matching_dbs.len() {
            0 => anyhow::bail!("Default database '{}' not found", self.default),
            1 => Ok(matching_dbs[0].clone()),
            _ => anyhow::bail!("Multiple databases with name '{}' found", self.default),
        }
    }
}
