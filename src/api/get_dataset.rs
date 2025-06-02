use serde::Serialize;

use crate::dataset::Dataset;

/// Request of [`crate::client::Client::get_dataset`]
#[derive(Debug, Serialize)]
pub struct GetDatasetReq {
  #[serde(rename = "repo_id")]
  pub(crate) repo_name: String,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) revision: Option<String>,
}

impl GetDatasetReq {
  pub fn new(repo_name: impl Into<String>) -> Self {
    GetDatasetReq {
      repo_name: repo_name.into(),
      revision: None,
    }
  }

  pub fn revision(mut self, revision: impl Into<String>) -> Self {
    self.revision = Some(revision.into());
    self
  }
}

impl<T: Into<String>> From<T> for GetDatasetReq {
  fn from(s: T) -> Self {
    Self {
      repo_name: s.into(),
      revision: None,
    }
  }
}

/// Response of [`crate::client::Client::get_dataset`]
pub type GetDatasetRes = Dataset;
