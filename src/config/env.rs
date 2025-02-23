use std::sync::LazyLock;  
use crate::validators::env_validator::EnvConfig;

pub static APP_CONFIG: LazyLock<EnvConfig> = LazyLock::new(|| EnvConfig::from_env().unwrap());
