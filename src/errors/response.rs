use super::types::AppError;
use actix_web::{
  body::BoxBody, http::StatusCode, middleware::ErrorHandlerResponse, HttpResponse, ResponseError,
  Result,
};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
  pub success: bool,
  pub message: String,
}

impl AppError {
  fn error_response(&self, status: StatusCode, show_internal: bool) -> HttpResponse {
    let message = if show_internal {
      self.to_string()
    } else {
      "Internal server error".into()
    };

    HttpResponse::build(status).json(ErrorResponse {
      success: false,
      message,
    })
  }
}

impl fmt::Display for AppError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      AppError::DbError(msg) => write!(f, "Database error: {}", msg),
      AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
      AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
      AppError::InternalServerError(msg) => write!(f, "Internal server error: {}", msg),
      AppError::Conflict(msg) => write!(f, "Conflict: {}", msg),
      AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
      AppError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
      AppError::ServiceUnavailable(msg) => write!(f, "Service Unavailable: {}", msg),
      AppError::RateLimitExceeded(msg) => write!(f, "Rate limit exceeded: {}", msg),
    }
  }
}

impl ResponseError for AppError {
  fn error_response(&self) -> HttpResponse {
    let (status, show_internal, level) = self.properties();
    log::log!(level, "{}", self);
    self.error_response(status, show_internal)
  }
}

pub fn not_found_handler(
  res: actix_web::dev::ServiceResponse,
) -> Result<ErrorHandlerResponse<BoxBody>> {
  let response = HttpResponse::NotFound().json(ErrorResponse {
    success: false,
    message: format!("Not Found"),
  });

  Ok(ErrorHandlerResponse::Response(
    res.into_response(response).map_into_left_body(),
  ))
}
