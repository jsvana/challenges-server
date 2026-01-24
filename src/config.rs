// src/config.rs
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub admin_token: String,
    pub port: u16,
    pub base_url: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| ConfigError::Missing("DATABASE_URL"))?;

        let admin_token = env::var("ADMIN_TOKEN")
            .map_err(|_| ConfigError::Missing("ADMIN_TOKEN"))?;

        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .map_err(|_| ConfigError::Invalid("PORT must be a number"))?;

        let base_url = env::var("BASE_URL").ok();

        Ok(Self {
            database_url,
            admin_token,
            port,
            base_url,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Missing required environment variable: {0}")]
    Missing(&'static str),
    #[error("Invalid configuration: {0}")]
    Invalid(&'static str),
}
