use serde::Serialize;

/// Request of [`crate::client::Client::create_collection`]
#[derive(Debug, Serialize)]
pub struct CreateCollectionReq<'a> {
  title: &'a str,
  namespace: &'a str,

  #[serde(skip_serializing_if = "Option::is_none")]
  description: Option<&'a str>,

  // #[serde(skip_serializing_if = "Option::is_none")]
  // item: Option<CreateCollectionItem<'a>>,
  private: bool,
}

impl<'a> CreateCollectionReq<'a> {
  pub fn new(title: &'a str, namespace: &'a str, private: bool) -> Self {
    CreateCollectionReq {
      title,
      namespace,
      description: None,
      // item: None,
      private,
    }
  }

  pub fn description(mut self, description: &'a str) -> Self {
    self.description = Some(description);
    self
  }

  // pub fn item(mut self, item: CreateCollectionItem<'a>) -> Self {
  //   self.item = Some(item);
  //   self
  // }
}
