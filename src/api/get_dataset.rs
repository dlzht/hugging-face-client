use serde::Serialize;

use crate::dataset::Dataset;

/// Request of [`crate::client::Client::get_dataset`]
#[derive(Debug, Serialize)]
pub struct GetDatasetReq<'a> {
  #[serde(rename = "repo_id")]
  pub(crate) repo_name: &'a str,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) revision: Option<&'a str>,
}

impl<'a> GetDatasetReq<'a> {
  pub fn new(repo_name: &str) -> GetDatasetReq<'_> {
    GetDatasetReq {
      repo_name,
      revision: None,
    }
  }

  pub fn revision(mut self, revision: &'a str) -> Self {
    self.revision = Some(revision);
    self
  }
}

/// Response of [`crate::client::Client::get_dataset`]
pub type GetDatasetRes = Dataset;
