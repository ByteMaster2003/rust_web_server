pub mod auth_routes;

use actix_web::web;
use crate::middlewares::rate_limiter::{RateLimitConfig, create_rate_limiter};

pub fn api_config(cfg: &mut web::ServiceConfig) {
	cfg.service(
		web::scope("/auth")
			.wrap(create_rate_limiter(RateLimitConfig::Auth))
			.configure(auth_routes::config)
	);
}
