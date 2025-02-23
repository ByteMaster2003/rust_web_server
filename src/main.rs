use web_server::app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  app::run_server().await
}
