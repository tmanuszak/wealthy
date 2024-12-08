use color_eyre::eyre::Ok;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
}

#[derive(Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Deserialize, Clone)]
pub struct ServerConfig {
    pub ip: String,
    pub port: u16,
}

pub async fn get_config() -> color_eyre::Result<Config> {
    let config_file = tokio::fs::read_to_string("config.toml").await?;
    let config = toml::from_str(&config_file)?;
    Ok(config)
}
