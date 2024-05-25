use siwi::IntoResponse;
use siwi::Response;
use siwi::Result;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum Error {
  #[error("hyper error")]
  SiwiError(#[from] siwi::Error),
  #[error("json error")]
  JsonError(#[from] serde_json::Error),
  #[error("json error")]
  AnyError(#[from] anyhow::Error),
}

impl IntoResponse for Error {
  fn into_response(self) -> Result {
    let val = self.to_string();
    Response::with_status(500, val)
  }
}
