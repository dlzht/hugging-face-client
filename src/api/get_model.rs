use serde::Serialize;

use crate::model::Model;

/// Request of [`crate::client::Client::get_model`]
#[derive(Debug, Serialize)]
pub struct GetModelReq<'a> {
  #[serde(rename = "repo_id")]
  pub(crate) name: &'a str,

  pub(crate) revision: Option<&'a str>,
}

impl<'a> GetModelReq<'a> {
  pub fn new(name: &str) -> GetModelReq<'_> {
    GetModelReq {
      name,
      revision: None,
    }
  }

  pub fn name(mut self, revision: &'a str) -> Self {
    self.revision = Some(revision);
    self
  }
}

/// Response of [`crate::client::Client::get_model`]
pub type GetModelRes = Model;
