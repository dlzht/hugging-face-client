use serde::Serialize;

use crate::RepoType;

/// Request of [`crate::client::Client::move_repo`]
#[derive(Debug, Serialize)]
pub struct MoveRepoReq<'a> {
  #[serde(rename = "fromRepo")]
  from_repo: &'a str,

  #[serde(rename = "toRepo")]
  to_repo: &'a str,

  #[serde(rename = "type")]
  repo_type: RepoType,
}

impl<'a> MoveRepoReq<'a> {
  pub fn new(from_repo: &'a str, to_repo: &'a str) -> Self {
    MoveRepoReq {
      from_repo,
      to_repo,
      repo_type: RepoType::default(),
    }
  }

  pub fn repo_type(mut self, repo_type: RepoType) -> Self {
    self.repo_type = repo_type;
    self
  }
}
