use web_server::{app, config::APP_CONFIG};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  println!("Server starting on {}:{}", APP_CONFIG.host, APP_CONFIG.port);

  app::run_server(APP_CONFIG.host.clone(), APP_CONFIG.port).await
}
