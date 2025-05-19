use serde::Serialize;

/// Request of [`crate::client::Client::create_collection`]
#[derive(Debug, Serialize)]
pub struct CreateCollectionReq {
  title: String,
  namespace: String,

  #[serde(skip_serializing_if = "Option::is_none")]
  description: Option<String>,

  // #[serde(skip_serializing_if = "Option::is_none")]
  // item: Option<CreateCollectionItem<'a>>,
  private: bool,
}

impl CreateCollectionReq {
  pub fn new(title: impl Into<String>, namespace: impl Into<String>, private: bool) -> Self {
    CreateCollectionReq {
      title: title.into(),
      namespace: namespace.into(),
      description: None,
      // item: None,
      private,
    }
  }

  pub fn description(mut self, description: impl Into<String>) -> Self {
    self.description = Some(description.into());
    self
  }

  // pub fn item(mut self, item: CreateCollectionItem<'a>) -> Self {
  //   self.item = Some(item);
  //   self
  // }
}

impl<T: Into<String>, U: Into<String>> From<(T, U, bool)> for CreateCollectionReq {
  fn from(value: (T, U, bool)) -> Self {
    Self {
      title: value.0.into(),
      namespace: value.1.into(),
      description: None,
      private: value.2,
    }
  }
}
