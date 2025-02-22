use crate::routes;
use actix_web::{web, App, HttpServer};

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(web::scope("/api/v1").configure(routes::v1::api_config));
}

pub async fn run_server(host: String, port: u16) -> std::io::Result<()> {
  HttpServer::new(|| App::new().configure(config))
    .bind((host, port))?
    .run()
    .await
}
