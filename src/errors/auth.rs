use super::types::AppError;

// Add any auth-specific error conversions here
// For example:
impl From<jwt::Error> for AppError {
  fn from(error: jwt::Error) -> Self {
    AppError::Unauthorized(error.to_string())
  }
}
