use serde::Serialize;

use crate::{RepoType, collection::Collection};

/// Request of [`crate::client::Client::create_collection_item`]
#[derive(Debug, Serialize)]
pub struct CreateCollectionItemReq<'a> {
  #[serde(skip_serializing)]
  pub(crate) collection_slug: &'a str,

  pub(crate) item: CreateCollectionItemInner<'a>,
  pub(crate) note: Option<&'a str>,
}

#[derive(Debug, Serialize)]
struct CreateCollectionItemInner<'a> {
  #[serde(rename = "type")]
  item_type: RepoType,

  id: &'a str,
}

impl<'a> CreateCollectionItemReq<'a> {
  pub fn new(collection_slug: &'a str, item_id: &'a str, item_type: RepoType) -> Self {
    let item = CreateCollectionItemInner {
      item_type,
      id: item_id,
    };
    CreateCollectionItemReq {
      collection_slug,
      item,
      note: None,
    }
  }

  pub fn note(mut self, note: &'a str) -> Self {
    self.note = Some(note);
    self
  }
}

/// Response of [`crate::client::Client::create_collection_item`]
pub type CreateCollectionItemRes = Collection;
