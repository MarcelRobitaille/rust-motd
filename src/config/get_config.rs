use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

use crate::config::Config;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error(
        "Configuration file not found.\n\
        Make a copy of default config and either specify it as an arg or \n\
        place it in a default location.  See ReadMe for details."
    )]
    ConfigNotFound,

    #[error(transparent)]
    ConfigHomeError(#[from] std::env::VarError),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    ConfigParseError(#[from] toml::de::Error),
}

pub fn get_config(config_path: Option<String>) -> Result<Config, ConfigError> {
    let config_path = match config_path {
        Some(file_path) => Some(PathBuf::from(file_path)),
        None => {
            let config_base = env::var("XDG_CONFIG_HOME").unwrap_or(env::var("HOME")? + "/.config");
            let config_base = Path::new(&config_base).join(Path::new("rust-motd/config.toml"));
            if config_base.exists() {
                Some(config_base)
            } else {
                None
            }
        }
    };

    let config_path = config_path.ok_or(ConfigError::ConfigNotFound)?;
    let toml_str = fs::read_to_string(config_path)?;
    Ok(toml::from_str(&toml_str)?)
}
