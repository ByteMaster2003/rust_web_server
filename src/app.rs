use crate::{
  config::APP_CONFIG,
  errors::{not_found_handler, AppError},
  routes,
  utils::{cors::configure_cors, db, logger}
};
use actix_web::{
  http::StatusCode,
  middleware::{Compress, DefaultHeaders, ErrorHandlers},
  web, App, HttpServer,
};
use log::info;

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(web::scope("/api/v1").configure(routes::v1::api_config));
}

pub async fn run_server() -> Result<(), AppError> {
  // Initialize logger
  logger::init_logger();

  // Connect to MongoDB
  let db_client = db::mongo_connect(&APP_CONFIG.mongo_uri).await?;
  let db_data = web::Data::new(db_client);

  // Start the server
  info!(
    "Server starting on http://{}:{}",
    APP_CONFIG.host, APP_CONFIG.port
  );

  HttpServer::new(move || {
    App::new()
      .wrap(ErrorHandlers::new().handler(StatusCode::NOT_FOUND, not_found_handler))
      .wrap(
        DefaultHeaders::new()
          .add(("X-Frame-Options", "DENY"))
          .add(("X-Content-Type-Options", "nosniff"))
          .add(("X-XSS-Protection", "1; mode=block"))
          .add(("Referrer-Policy", "strict-origin-when-cross-origin")),
      )
      .wrap(configure_cors())
      .wrap(Compress::default())
      .wrap(logger::get_logger_middleware())
      .app_data(db_data.clone())
      .configure(config)
  })
  .bind((APP_CONFIG.host.clone(), APP_CONFIG.port))?
  .run()
  .await
  .map_err(|e| AppError::InternalServerError(e.to_string()))
}
