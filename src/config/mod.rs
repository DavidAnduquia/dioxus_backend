use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)] 
pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub jwt_secret: String,
    pub environment: Environment,
}

#[derive(Debug, Clone, Deserialize)]
pub enum Environment {
    Development,
    Production,
    Testing,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenvy::dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://localhost/rust_api_db".to_string());

        let port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()?;

        let jwt_secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| "your-secret-key-change-in-production".to_string());

        let environment = match env::var("ENVIRONMENT")
            .unwrap_or_else(|_| "development".to_string())
            .to_lowercase()
            .as_str()
        {
            "production" => Environment::Production,
            "testing" => Environment::Testing,
            _ => Environment::Development,
        };

        Ok(Config {
            database_url,
            port,
            jwt_secret,
            environment,
        })
    }
}
