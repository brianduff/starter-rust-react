use std::fmt::Display;

use axum::response::IntoResponse;


pub type Result<T> = std::result::Result<T, MyError>;

#[derive(Debug)]
pub enum MyError {
  Failed(anyhow::Error),
}

impl Display for MyError {
  fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::result::Result::Ok(())
  }
}

impl From<anyhow::Error> for MyError {
  fn from(err: anyhow::Error) -> MyError {
    MyError::Failed(err)
  }
}

impl IntoResponse for MyError {
  fn into_response(self) -> axum::response::Response {
    match self {
      MyError::Failed(e) => {
        tracing::error!("Internal error: {:?}", e);
        (
          axum::http::StatusCode::INTERNAL_SERVER_ERROR,
          format!("An internal error occurred: {:?}", e),
        ).into_response()
      }
    }
  }
}