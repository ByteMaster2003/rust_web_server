use dotenv::dotenv;
use serde::Deserialize;
use thiserror::Error;
use validator::Validate;

#[derive(Error, Debug)]
pub enum EnvConfigError {
  #[error("Environment variable not found: {0}")]
  EnvVarMissing(String),

  #[error("Validation error: {0}")]
  ValidationError(String),

  #[error("Failed to parse environment variable: {0}")]
  ParseError(String),
}

#[derive(Debug, Deserialize, Validate)]
pub struct EnvConfig {
  #[validate(range(min = 1, max = 65535))]
  pub port: u16,

  #[validate(length(min = 1))]
  pub host: String,

  #[validate(length(min = 1))]
  pub mongo_uri: String,
}

impl EnvConfig {
  pub fn from_env() -> Result<Self, EnvConfigError> {
    dotenv().ok();

    let config = EnvConfig {
      // Server
      host: Self::get_env("HOST")?,
      port: Self::get_env("PORT")?
        .parse()
        .map_err(|_| EnvConfigError::ParseError("PORT".to_string()))?,

      // MongoDB
      mongo_uri: Self::get_env("MONGO_URI")?,
    };

    // Validate all fields
    if let Err(errors) = config.validate() {
      return Err(EnvConfigError::ValidationError(errors.to_string()));
    }

    Ok(config)
  }

  fn get_env(key: &str) -> Result<String, EnvConfigError> {
    std::env::var(key).map_err(|_| EnvConfigError::EnvVarMissing(key.to_string()))
  }
}
