use crate::{config::APP_CONFIG, routes, utils::db_util};
use actix_web::{web, App, HttpServer};

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(web::scope("/api/v1").configure(routes::v1::api_config));
}

pub async fn run_server() -> std::io::Result<()> {
  let db_client = db_util::db_connect(&APP_CONFIG.mongo_uri).await?;
  let db_data = web::Data::new(db_client);

  println!("Server starting on {}:{}", APP_CONFIG.host, APP_CONFIG.port);
  HttpServer::new(move || App::new().app_data(db_data.clone()).configure(config))
    .bind((APP_CONFIG.host.clone(), APP_CONFIG.port))?
    .run()
    .await
}
