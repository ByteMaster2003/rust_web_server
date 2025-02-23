use super::types::AppError;
use validator::ValidationErrors;

impl From<ValidationErrors> for AppError {
  fn from(error: ValidationErrors) -> Self {
    AppError::BadRequest(error.to_string())
  }
}
