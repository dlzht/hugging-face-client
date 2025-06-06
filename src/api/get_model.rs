use serde::Serialize;

use crate::model::Model;

/// Request of [`crate::client::Client::get_model`]
#[derive(Debug, Serialize)]
pub struct GetModelReq<'a> {
  #[serde(rename = "repo_id")]
  pub(crate) repo_name: &'a str,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) revision: Option<&'a str>,
}

impl<'a> GetModelReq<'a> {
  pub fn new(repo_name: &str) -> GetModelReq<'_> {
    GetModelReq {
      repo_name,
      revision: None,
    }
  }

  pub fn revision(mut self, revision: &'a str) -> Self {
    self.revision = Some(revision);
    self
  }
}

/// Response of [`crate::client::Client::get_model`]
pub type GetModelRes = Model;
