use serde::{Deserialize, Serialize};

/// Request of [`crate::client::Client::delete_collection`]
#[derive(Debug, Serialize)]
pub struct DeleteCollectionReq {
  pub(crate) slug: String,
}

impl DeleteCollectionReq {
  pub fn new(slug: impl Into<String>) -> DeleteCollectionReq {
    DeleteCollectionReq { slug: slug.into() }
  }
}

impl<T: Into<String>> From<T> for DeleteCollectionReq {
  fn from(value: T) -> Self {
    Self { slug: value.into() }
  }
}

/// Response of [`crate::client::Client::delete_collection`]
#[derive(Debug, Deserialize)]
pub struct DeleteCollectionRes {
  pub success: bool,
}
