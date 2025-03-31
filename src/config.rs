use std::env;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    #[allow(dead_code)]
    pub jwt_expires_in: String,
    pub port: u16,
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Variável de ambiente não encontrada: {0}")]
    NotFound(String),
    
    #[error("Falha ao converter variável de ambiente: {0}")]
    ParseError(String),
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| ConfigError::NotFound("DATABASE_URL".to_string()))?;
            
        let jwt_secret = env::var("JWT_SECRET")
            .map_err(|_| ConfigError::NotFound("JWT_SECRET".to_string()))?;
            
        let jwt_expires_in = env::var("JWT_EXPIRES_IN")
            .unwrap_or_else(|_| "24h".to_string());
            
        let port = env::var("PORT")
            .unwrap_or_else(|_| "50001".to_string())
            .parse::<u16>()
            .map_err(|_| ConfigError::ParseError("PORT".to_string()))?;
            
        Ok(Self {
            database_url,
            jwt_secret,
            jwt_expires_in,
            port,
        })
    }
} 