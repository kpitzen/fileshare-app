use std::fs::File;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};

extern crate dirs;
extern crate toml;


#[derive(Deserialize, Debug, Serialize)]
pub struct Config {
    pub postgres: PostgresConfig,
}


#[derive(Deserialize, Debug, Serialize)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}


pub fn get_config() -> std::io::Result<Config> {
    let config_path = format!(
        "{home_dir}/.config/fileshare-app.toml",
        home_dir = dirs::home_dir().unwrap().to_str().unwrap()
    );
    let mut file = File::open(config_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: Config = toml::from_str(&contents).unwrap();
    Ok(config)
}


#[cfg(test)]
mod tests {
    use super::get_config;
    
    #[test]
    fn test_get_config() {
        assert!(get_config().is_ok(), "Unable to get config!");
    }
}
