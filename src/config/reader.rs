use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use toml;

#[derive(Deserialize, Debug, Serialize)]
pub struct Config {
    pub app_config: AppConfig,
    pub postgres: PostgresConfig,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

pub fn get_config(config_path: String) -> std::io::Result<Config> {
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
        assert!(
            get_config(String::from(".sample_config.toml")).is_ok(),
            "Unable to get config!"
        );
    }
}
