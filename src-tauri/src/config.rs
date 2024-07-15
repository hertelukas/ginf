use std::{fs, path::PathBuf};

use dirs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub folder: String,
    pub tags: Vec<String>,
}

impl Config {
    pub fn new() -> Self {
        let config = match fs::read_to_string(Config::config_path()) {
            Ok(content) => content,
            Err(_error) => panic!("Could not read from config."),
        };

        let config: Config = match toml::from_str(&config) {
            Ok(content) => content,
            Err(_error) => panic!("Config parsing failed."),
        };

        config
    }

    fn config_path() -> PathBuf {
        if let Some(config_dir) = dirs::config_dir() {
            config_dir.join("ginf").join("config.toml")
        } else {
            panic!("Config directory not found.")
        }
    }

    pub fn db_path(&self) -> PathBuf {
        PathBuf::from(&self.folder).join("files.sqlite")
    }

    pub fn folder_path(&self) -> PathBuf {
        PathBuf::from(&self.folder)
    }
}
