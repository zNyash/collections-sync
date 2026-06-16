use std::{fs, path::PathBuf};

use anyhow::{Context, anyhow};
use inquire::{prompt_text, prompt_u64};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub client_id: u64,
    pub client_secret: String,
    pub osu_path: PathBuf,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            let config_instance = Self::create_config(&config_path)?;

            return Ok(config_instance);
        }

        let config_instance = Self::read_config(&config_path)?;

        Ok(config_instance)
    }

    fn config_path() -> anyhow::Result<PathBuf> {
        let dirs = directories::ProjectDirs::from("com", "NSH", "collections-sync")
            .context("Failed to get config directory for this system.")?;

        let config_dir = dirs.config_dir();

        if !config_dir.exists() {
            fs::create_dir_all(config_dir).context("Failed to create config directory.")?;
        }

        Ok(config_dir.join("config.json"))
    }

    fn create_config(config_path: &PathBuf) -> anyhow::Result<Self> {
        let client_id = prompt_u64("Your Client ID:")?;
        let client_secret = prompt_text("Your Client Secret:")?;
        let osu_path_input = prompt_text("Your osu folder path:")?;
        let osu_path = PathBuf::from(shellexpand::tilde(&osu_path_input).as_ref());

        let config_instance = Self {
            client_id,
            client_secret,
            osu_path,
        };

        config_instance.validate_config()?;

        fs::write(config_path, serde_json::to_string_pretty(&config_instance)?)?;

        Ok(config_instance)
    }

    fn read_config(config_path: &PathBuf) -> anyhow::Result<Self> {
        let config_contents = fs::read_to_string(config_path)?;
        let config_instance: Self = serde_json::from_str(&config_contents)?;
        config_instance.validate_config()?;

        Ok(config_instance)
    }

    fn validate_config(&self) -> anyhow::Result<()> {
        // Fields validation
        if self.client_id == 0 {
            return Err(anyhow!("client_id is empty."));
        }
        if self.client_secret.trim().is_empty() {
            return Err(anyhow!("client_secret is empty."));
        }

        // Path validation
        if !self.osu_path.exists() {
            return Err(anyhow!(
                "osu path does not exists or is invalid: {:?}",
                self.osu_path
            ));
        }
        if !self.osu_path.join("Songs").exists() {
            return Err(anyhow!("Could not find Songs folder inside osu_path."));
        }
        if !self.osu_path.join("osu!.db").exists() {
            return Err(anyhow!("There's no osu!.db file inside the osu! folder."));
        }
        if !self.osu_path.join("collection.db").exists() {
            return Err(anyhow!(
                "There's no collection.db file inside the osu! folder."
            ));
        }

        Ok(())
    }
}
