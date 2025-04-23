use serde::Serialize;

use crate::RepoType;

/// Request of [`crate::client::Client::delete_repo`]
#[derive(Debug, Serialize)]
pub struct DeleteRepoReq<'a> {
  name: &'a str,

  #[serde(rename = "type")]
  repo_type: RepoType,

  #[serde(skip_serializing_if = "Option::is_none")]
  organization: Option<&'a str>,
}

impl<'a> DeleteRepoReq<'a> {
  pub fn new(name: &'a str) -> Self {
    DeleteRepoReq {
      name,
      repo_type: RepoType::default(),
      organization: None,
    }
  }

  pub fn repo_type(mut self, repo_type: RepoType) -> Self {
    self.repo_type = repo_type;
    self
  }

  pub fn organization(mut self, organization: &'a str) -> Self {
    self.organization = Some(organization);
    self
  }
}

// Response of [`crate::client::Client::delete_repo`]
// #[derive(Debug, Deserialize)]
// pub struct DeleteRepoRes {
//
// }
