use color_eyre::eyre::Ok;
use serde::Deserialize;
use tap::prelude::*;

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
    Config {
        database: DatabaseConfig {
            url: dotenvy::var("DATABASE_URL")?,
        },
        server: ServerConfig {
            ip: dotenvy::var("SERVER_IP")?,
            port: dotenvy::var("SERVER_PORT")?.parse::<u16>()?,
        },
    }
    .pipe(Ok)
}
