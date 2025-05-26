use serde::Serialize;

use crate::RepoType;

/// Request of [`crate::client::Client::delete_repo`]
#[derive(Debug, Serialize)]
pub struct DeleteRepoReq {
  name: String,

  #[serde(rename = "type")]
  repo_type: RepoType,

  #[serde(skip_serializing_if = "Option::is_none")]
  organization: Option<String>,
}

impl DeleteRepoReq {
  pub fn new(name: impl Into<String>) -> Self {
    DeleteRepoReq {
      name: name.into(),
      repo_type: RepoType::default(),
      organization: None,
    }
  }

  pub fn repo_type(mut self, repo_type: RepoType) -> Self {
    self.repo_type = repo_type;
    self
  }

  pub fn organization(mut self, organization: impl Into<String>) -> Self {
    self.organization = Some(organization.into());
    self
  }
}

impl<T: Into<String>> From<T> for DeleteRepoReq {
  fn from(value: T) -> Self {
    Self {
      name: value.into(),
      repo_type: Default::default(),
      organization: None,
    }
  }
}

// Response of [`crate::client::Client::delete_repo`]
// #[derive(Debug, Deserialize)]
// pub struct DeleteRepoRes {
//
// }
