use serde::{Deserialize, Serialize};

use crate::collection::Collection;

/// Request of [`crate::client::Client::modify_collection`]
#[derive(Debug, Serialize)]
pub struct ModifyCollectionReq<'a> {
  #[serde(skip_serializing)]
  pub(crate) slug: &'a str,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) title: Option<&'a str>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) description: Option<&'a str>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) private: Option<bool>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) position: Option<usize>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) theme: Option<&'a str>,
}

impl<'a> ModifyCollectionReq<'a> {
  pub fn new(slug: &'a str) -> ModifyCollectionReq<'a> {
    ModifyCollectionReq {
      slug,
      title: None,
      description: None,
      private: None,
      position: None,
      theme: None,
    }
  }

  pub fn title(mut self, title: &'a str) -> Self {
    self.title = Some(title);
    self
  }

  pub fn description(mut self, description: &'a str) -> Self {
    self.description = Some(description);
    self
  }

  pub fn private(mut self, private: bool) -> Self {
    self.private = Some(private);
    self
  }

  pub fn position(mut self, position: usize) -> Self {
    self.position = Some(position);
    self
  }

  pub fn theme(mut self, theme: &'a str) -> Self {
    self.theme = Some(theme);
    self
  }
}

/// Response of [`crate::client::Client::modify_collection`]
#[derive(Debug, Deserialize)]
pub struct ModifyCollectionRes {
  pub success: bool,
  pub data: Collection,
}
