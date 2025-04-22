use serde::Serialize;

use crate::model::Model;

/// Request of [`crate::client::Client::get_model`]
#[derive(Debug, Serialize)]
pub struct GetModelReq<'a> {
  pub(crate) repo_id: &'a str,
  pub(crate) revision: Option<&'a str>,
}

impl<'a> GetModelReq<'a> {
  pub fn new(repo_id: &str) -> GetModelReq<'_> {
    GetModelReq {
      repo_id,
      revision: None,
    }
  }

  pub fn repo_id(mut self, revision: &'a str) -> Self {
    self.revision = Some(revision);
    self
  }
}

/// Response of [`crate::client::Client::get_model`]
pub type GetModelRes = Model;
