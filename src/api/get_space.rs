use serde::Serialize;

use crate::space::Space;

/// Request of [`crate::client::Client::get_space`]
#[derive(Debug, Serialize)]
pub struct GetSpaceReq<'a> {
  #[serde(rename = "repo_id")]
  pub(crate) repo_name: &'a str,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub(crate) revision: Option<&'a str>,
}

impl<'a> GetSpaceReq<'a> {
  pub fn new(repo_name: &str) -> GetSpaceReq<'_> {
    GetSpaceReq {
      repo_name,
      revision: None,
    }
  }

  pub fn revision(mut self, revision: &'a str) -> Self {
    self.revision = Some(revision);
    self
  }
}

/// Response of [`crate::client::Client::get_space`]
pub type GetSpaceRes = Space;
