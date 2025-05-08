use serde::{Deserialize, Serialize};

/// Request of [`crate::client::Client::delete_collection_item`]
#[derive(Debug, Serialize)]
pub struct DeleteCollectionItemReq<'a> {
  #[serde(skip_serializing)]
  pub(crate) collection_slug: &'a str,

  #[serde(skip_serializing)]
  pub(crate) item_id: &'a str,
}

impl<'a> DeleteCollectionItemReq<'a> {
  pub fn new(collection_slug: &'a str, item_id: &'a str) -> Self {
    Self {
      collection_slug,
      item_id,
    }
  }
}

/// Response of [`crate::client::Client::delete_collection_item`]
#[derive(Debug, Deserialize)]
pub struct DeleteCollectionItemRes {
  pub success: bool,
}
