use serde::Serialize;

use crate::{RepoType, collection::Collection};

/// Request of [`crate::client::Client::create_collection_item`]
#[derive(Debug, Serialize)]
pub struct CreateCollectionItemReq {
  #[serde(skip_serializing)]
  pub(crate) collection_slug: String,

  item: CreateCollectionItemInner,

  pub(crate) note: Option<String>,
}

#[derive(Debug, Serialize)]
struct CreateCollectionItemInner {
  #[serde(rename = "type")]
  item_type: RepoType,

  id: String,
}

impl<'a> CreateCollectionItemReq {
  pub fn new(collection_slug: impl Into<String>, item_id: impl Into<String>, item_type: RepoType) -> Self {
    let item = CreateCollectionItemInner {
      item_type,
      id: item_id.into(),
    };
    CreateCollectionItemReq {
      collection_slug: collection_slug.into(),
      item,
      note: None,
    }
  }

  pub fn note(mut self, note: impl Into<String>) -> Self {
    self.note = Some(note.into());
    self
  }
}

impl<T: Into<String>, U: Into<String>> From<(T, U, RepoType)> for CreateCollectionItemReq {
  fn from(value: (T, U, RepoType)) -> Self {
    let item = CreateCollectionItemInner {
      item_type: value.2,
      id: value.1.into(),
    };
    CreateCollectionItemReq {
      collection_slug: value.0.into(),
      item,
      note: None,
    }
  }
}

/// Response of [`crate::client::Client::create_collection_item`]
pub type CreateCollectionItemRes = Collection;
