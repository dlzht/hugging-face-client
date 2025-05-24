use serde::{Deserialize, Serialize};

/// Request of [`crate::client::Client::delete_collection_item`]
#[derive(Debug, Serialize)]
pub struct DeleteCollectionItemReq {
  #[serde(skip_serializing)]
  pub(crate) collection_slug: String,

  #[serde(skip_serializing)]
  pub(crate) item_id: String,
}

impl DeleteCollectionItemReq {
  pub fn new(collection_slug: impl Into<String>, item_id: impl Into<String>) -> Self {
    Self {
      collection_slug: collection_slug.into(),
      item_id: item_id.into(),
    }
  }
}

impl<T: Into<String>, U: Into<String>> From<(T, U)> for DeleteCollectionItemReq {
  fn from(value: (T, U)) -> Self {
    Self {
      collection_slug: value.0.into(),
      item_id: value.1.into(),
    }
  }
}

/// Response of [`crate::client::Client::delete_collection_item`]
#[derive(Debug, Deserialize)]
pub struct DeleteCollectionItemRes {
  pub success: bool,
}
