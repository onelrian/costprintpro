use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub port: u16,
    pub cors_origins: Vec<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv::dotenv().ok();

        let config = Config {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://postgres:password@localhost/costprint".to_string()),
            redis_url: env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your-secret-key-change-in-production".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3001".to_string())
                .parse()
                .unwrap_or(3001),
            cors_origins: env::var("CORS_ORIGINS")
                .unwrap_or_else(|_| "http://localhost:3000".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
        };

        Ok(config)
    }
}
