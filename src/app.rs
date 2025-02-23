use crate::{config::APP_CONFIG, routes, utils::{db, logger}};
use actix_web::{web, App, HttpServer};
use log::info;

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(web::scope("/api/v1").configure(routes::v1::api_config));
}

pub async fn run_server() -> std::io::Result<()> {
	// Initialize logger
	logger::init_logger();

	// Connect to MongoDB
  let db_client = db::mongo_connect(&APP_CONFIG.mongo_uri).await?;
  let db_data = web::Data::new(db_client);

	// Start the server
	info!("Server starting on http://{}:{}", APP_CONFIG.host, APP_CONFIG.port);
  HttpServer::new(move || App::new().app_data(db_data.clone()).configure(config))
    .bind((APP_CONFIG.host.clone(), APP_CONFIG.port))?
    .run()
    .await
}
