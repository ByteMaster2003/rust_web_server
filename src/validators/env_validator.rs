use dotenv::dotenv;
use serde::Deserialize;
use thiserror::Error;
use validator::Validate;

#[derive(Error, Debug)]
pub enum AppConfigError {
  #[error("Environment variable not found: {0}")]
  EnvVarMissing(String),

  #[error("Validation error: {0}")]
  ValidationError(String),

  #[error("Failed to parse environment variable: {0}")]
  ParseError(String),
}

#[derive(Debug, Deserialize, Validate)]
pub struct AppConfig {
  #[validate(range(min = 1, max = 65535))]
  pub port: u16,

  #[validate(length(min = 1))]
  pub host: String,

  #[validate(length(min = 1))]
  pub mongo_uri: String,

  #[validate(length(min = 1))]
  pub allowed_origins: Vec<String>,
}

impl AppConfig {
  pub fn init() -> Result<Self, AppConfigError> {
    dotenv().ok();

    let config = AppConfig {
      // Server
      host: Self::get_env("HOST")?,
      port: Self::get_env("PORT")?
        .parse()
        .map_err(|_| AppConfigError::ParseError("PORT".to_string()))?,

      // MongoDB
      mongo_uri: Self::get_env("MONGO_URI")?,
      allowed_origins: Self::get_env("ALLOWED_ORIGINS")?
        .split(',')
        .map(|s| s.trim().to_string())
        .collect(),
    };

    // Validate all fields
    if let Err(errors) = config.validate() {
      return Err(AppConfigError::ValidationError(errors.to_string()));
    }

    Ok(config)
  }

  fn get_env(key: &str) -> Result<String, AppConfigError> {
    std::env::var(key).map_err(|_| AppConfigError::EnvVarMissing(key.to_string()))
  }
}
