use serde::{Deserialize, Serialize};

use crate::collection::CollectionItem;

/// Request of [`crate::client::Client::modify_collection_item`]
#[derive(Debug, Serialize)]
pub struct ModifyCollectionItemReq<'a> {
  #[serde(skip_serializing)]
  pub(crate) collection_slug: &'a str,

  #[serde(skip_serializing)]
  pub(crate) item_id: &'a str,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) position: Option<usize>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) note: Option<&'a str>,
}

impl<'a> ModifyCollectionItemReq<'a> {
  pub fn new(collection_slug: &'a str, item_id: &'a str) -> Self {
    Self {
      collection_slug,
      item_id,
      position: None,
      note: None,
    }
  }

  pub fn position(mut self, position: usize) -> Self {
    self.position = Some(position);
    self
  }

  pub fn note(mut self, note: &'a str) -> Self {
    self.note = Some(note);
    self
  }
}

/// Response of [`crate::client::Client::modify_collection_item`]
#[derive(Debug, Deserialize)]
pub struct ModifyCollectionItemRes {
  pub success: bool,
  pub data: CollectionItem,
}
