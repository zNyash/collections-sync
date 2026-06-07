use std::{fs, path::PathBuf};

use anyhow::{Context, Result};
use inquire::{CustomType, Text};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub client_id: u64,
    pub client_secret: String,
    pub osu_path: PathBuf,
}

pub fn load() -> Result<Config> {
    let path = default_config_path()?;

    if !path.exists() {
        return create(&path);
    }

    let text = fs::read_to_string(&path)?;
    let config: Config = serde_json::from_str(&text)?;

    Ok(config)
}

fn default_config_path() -> anyhow::Result<PathBuf> {
    let dirs = directories::ProjectDirs::from("com", "NSH", "collections-sync")
        .context("Failed to get config directoy.")?;

    let config_dir = dirs.config_dir();

    if !config_dir.exists() {
        fs::create_dir_all(config_dir).context("Failed to create config directory.")?;
    }

    Ok(config_dir.join("config.json"))
}

fn create(path: &PathBuf) -> Result<Config> {
    let client_id = CustomType::<u64>::new("Enter your osu! client ID:")
        .with_error_message("Please enter a valid number")
        .prompt()?;
    let client_secret = Text::new("Enter your osu! client secret:").prompt()?;
    let osu_path =
        Text::new("Enter the DIRECT (absolute) path to your osu! installation:").prompt()?;

    let config = Config {
        client_id,
        client_secret,
        osu_path: PathBuf::from(osu_path),
    };

    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(path, serde_json::to_string_pretty(&config)?)?;

    Ok(config)
}

pub fn validate(config: &Config) -> Result<()> {
    if !config.osu_path.exists() {
        anyhow::bail!("osu! path does not exist: {:?}", config.osu_path);
    }

    if !config.osu_path.join("osu!.db").exists() {
        anyhow::bail!("osu!.db not found in {:?}", config.osu_path);
    }

    if !config.osu_path.join("collection.db").exists() {
        anyhow::bail!("collection.db not found in {:?}", config.osu_path);
    }

    Ok(())
}
