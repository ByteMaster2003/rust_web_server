use super::types::AppError;
use mongodb::error::Error as MongoError;

impl From<MongoError> for AppError {
  fn from(error: MongoError) -> Self {
    AppError::DbError(error.to_string())
  }
}

impl From<std::io::Error> for AppError {
  fn from(error: std::io::Error) -> Self {
    AppError::InternalServerError(error.to_string())
  }
}
