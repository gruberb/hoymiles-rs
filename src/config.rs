use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, read_to_string, File};
use std::io::{Result, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub(crate) struct Config {
    pub(crate) hoymiles_token: String,
}

pub(crate) fn get_config_file() -> PathBuf {
    let mut home_path = home_dir().expect("Failed to get home directory");
    home_path.push(".config");
    home_path.push("hoymiles-rs");
    home_path.push("config.toml");
    home_path
}

pub(crate) fn write_config(config: &Config) -> Result<()> {
    let config_path = get_config_file();
    if let Some(parent) = config_path.parent() {
        create_dir_all(parent)?;
    }
    let mut file = File::create(config_path)?;
    let toml_content = toml::to_string(config).expect("Failed to serialize config");
    file.write_all(toml_content.as_bytes())?;
    Ok(())
}

pub(crate) fn read_config() -> Result<Config> {
    let config_path = get_config_file();
    let contents = read_to_string(config_path)?;
    let config: Config = toml::from_str(&contents).expect("Failed to parse config file");
    Ok(config)
}
