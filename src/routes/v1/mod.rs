pub mod auth_routes;

use actix_web::web;

pub fn api_config(cfg: &mut web::ServiceConfig) {
	cfg.configure(auth_routes::config);
}
