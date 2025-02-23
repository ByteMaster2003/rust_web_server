use std::sync::LazyLock;  
use crate::validators::env_validator::AppConfig;

pub static APP_CONFIG: LazyLock<AppConfig> = LazyLock::new(|| AppConfig::init().unwrap());
