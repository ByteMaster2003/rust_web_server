use actix_governor::{
  governor::middleware::StateInformationMiddleware, Governor, GovernorConfig,
  GovernorConfigBuilder, PeerIpKeyExtractor,
};
use std::time::Duration;

pub enum RateLimitConfig {
  Default,
  Api,
  Auth,
  Custom(GovernorConfig<PeerIpKeyExtractor, StateInformationMiddleware>),
}

impl RateLimitConfig {
  pub fn get_config(self) -> Governor<PeerIpKeyExtractor, StateInformationMiddleware> {
    let config = match self {
      RateLimitConfig::Default => GovernorConfigBuilder::default()
        .period(Duration::from_secs(600))
        .seconds_per_request(60)
				.burst_size(10)
        .use_headers()
        .finish()
        .unwrap(),

      RateLimitConfig::Api => GovernorConfigBuilder::default()
        .period(Duration::from_secs(900))
        .seconds_per_request(60)
				.burst_size(10)
        .use_headers()
        .finish()
        .unwrap(),

      RateLimitConfig::Auth => GovernorConfigBuilder::default()
        .period(Duration::from_secs(600)) 
				.seconds_per_request(60)
        .burst_size(3)
        .use_headers()
        .finish()
        .unwrap(),

      RateLimitConfig::Custom(config) => config,
    };

    Governor::new(&config)
  }
}

pub fn create_rate_limiter(
  config: RateLimitConfig,
) -> Governor<PeerIpKeyExtractor, StateInformationMiddleware> {
  config.get_config()
}
