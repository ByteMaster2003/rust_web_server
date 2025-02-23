use actix_web::http::StatusCode;
use log::Level;

#[derive(Debug)]
pub enum AppError {
  DbError(String),
  NotFound(String),
  BadRequest(String),
  InternalServerError(String),
  Conflict(String),
  Unauthorized(String),
  Forbidden(String),
  ServiceUnavailable(String),
  RateLimitExceeded(String),
}

impl AppError {
  pub fn properties(&self) -> (StatusCode, bool, Level) {
    match self {
      AppError::DbError(_) => (StatusCode::INTERNAL_SERVER_ERROR, false, Level::Error),
      AppError::NotFound(_) => (StatusCode::NOT_FOUND, true, Level::Warn),
      AppError::BadRequest(_) => (StatusCode::BAD_REQUEST, true, Level::Warn),
      AppError::InternalServerError(_) => (StatusCode::INTERNAL_SERVER_ERROR, false, Level::Error),
      AppError::Conflict(_) => (StatusCode::CONFLICT, true, Level::Warn),
      AppError::Unauthorized(_) => (StatusCode::UNAUTHORIZED, true, Level::Warn),
      AppError::Forbidden(_) => (StatusCode::FORBIDDEN, true, Level::Warn),
      AppError::ServiceUnavailable(_) => (StatusCode::SERVICE_UNAVAILABLE, true, Level::Error),
      AppError::RateLimitExceeded(_) => (StatusCode::TOO_MANY_REQUESTS, true, Level::Warn),
    }
  }
}
