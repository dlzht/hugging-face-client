use serde::Serialize;

use crate::api::create_collection_item::CreateCollectionItemReq;

/// Request of [`crate::client::Client::create_collection`]
#[derive(Debug, Serialize)]
pub struct CreateCollectionReq<'a> {
  title: &'a str,
  namespace: &'a str,

  #[serde(skip_serializing_if = "Option::is_none")]
  description: Option<&'a str>,

  #[serde(skip_serializing_if = "Option::is_none")]
  item: Option<CreateCollectionItemReq<'a>>,
  private: bool,
}

impl<'a> CreateCollectionReq<'a> {
  pub fn new(title: &'a str, namespace: &'a str, private: bool) -> Self {
    CreateCollectionReq {
      title,
      namespace,
      description: None,
      item: None,
      private,
    }
  }

  pub fn description(mut self, description: &'a str) -> Self {
    self.description = Some(description);
    self
  }

  pub fn item(mut self, item: CreateCollectionItemReq<'a>) -> Self {
    self.item = Some(item);
    self
  }
}
