use web_server::{app, errors::AppError};

#[actix_web::main]
async fn main() -> Result<(), AppError> {
    app::run_server().await
}
