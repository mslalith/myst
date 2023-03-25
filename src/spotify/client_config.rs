use std::{fs, path::PathBuf};

use anyhow::{anyhow, Ok, Result};
use serde::{Deserialize, Serialize};

pub const DEFAULT_CONFIG_PORT: u16 = 8888;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfig {
    pub client_id: String,
    pub client_secret: String,
    pub port: u16,
}

pub struct ConfigPaths {
    pub config_file_path: PathBuf,
    pub token_cache_path: PathBuf,
}

impl ClientConfig {
    pub fn new() -> ClientConfig {
        ClientConfig {
            client_id: "".to_string(),
            client_secret: "".to_string(),
            port: DEFAULT_CONFIG_PORT,
        }
    }

    pub fn get_config_paths() -> Result<ConfigPaths> {
        Ok(ConfigPaths {
            config_file_path: Self::get_client_config_file()?,
            token_cache_path: Self::get_token_cache_file()?,
        })
    }

    pub fn load_config() -> Result<ClientConfig> {
        let config_file = Self::get_client_config_file()?;
        let contents = fs::read_to_string(&config_file)?;
        let config: ClientConfig = serde_yaml::from_str(&contents)?;
        Ok(config)
    }

    pub fn save_config(&self) -> Result<()> {
        let config_file = Self::get_client_config_file()?;
        let contents = serde_yaml::to_string(&self)?;
        fs::write(config_file, contents)?;
        Ok(())
    }

    pub fn get_redirect_uri(port: u16) -> String {
        format!("{}/callback", Self::get_local_server_addr(port))
    }

    pub fn get_local_server_addr(port: u16) -> String {
        format!("http://localhost:{}", port)
    }

    fn get_config_dir() -> Result<PathBuf> {
        match dirs::home_dir() {
            Some(home_dir) => {
                let config_dir = home_dir.join(".config/myst");
                if !config_dir.exists() {
                    fs::create_dir_all(&config_dir)?;
                }
                Ok(config_dir)
            }
            None => Err(anyhow!("Home directory not found")),
        }
    }

    fn get_client_config_file() -> Result<PathBuf> {
        let config_dir = Self::get_config_dir()?;
        Ok(config_dir.join("client_config.yml"))
    }

    fn get_token_cache_file() -> Result<PathBuf> {
        let config_dir = Self::get_config_dir()?;
        Ok(config_dir.join(".spotify_token_cache.json"))
    }
}
