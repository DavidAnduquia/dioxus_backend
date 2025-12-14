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

        // Usar valores por defecto sin allocations innecesarias
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://localhost/rust_api_db".into());

        let port = env::var("PORT")
            .ok()
            .and_then(|p| p.parse::<u16>().ok())
            .unwrap_or(3000);

        let jwt_secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| "your-secret-key-change-in-production".into());

        let environment = env::var("ENVIRONMENT")
            .ok()
            .and_then(|e| match e.to_lowercase().as_str() {
                "production" => Some(Environment::Production),
                "testing" => Some(Environment::Testing),
                "development" => Some(Environment::Development),
                _ => None,
            })
            .unwrap_or(Environment::Development);

        Ok(Config {
            database_url,
            port,
            jwt_secret,
            environment,
        })
    }
}
