use actix_cors::Cors;
use actix_web::http::{header, Method};

pub fn configure_cors() -> Cors {
  let mut cors = Cors::default()
    .allowed_methods(vec![
      Method::GET,
      Method::POST,
      Method::PATCH,
      Method::PUT,
      Method::DELETE,
    ])
    .allowed_headers(vec![
      header::CONTENT_TYPE,
      header::AUTHORIZATION,
      header::ACCEPT,
    ])
    .supports_credentials();

  // Production mode - use configured origins
  for origin in &crate::config::APP_CONFIG.allowed_origins {
    cors = cors.allowed_origin(origin);
  }

  cors
}
