use serde::Serialize;

use crate::model::Model;

/// Request of [`crate::client::Client::get_model`]
#[derive(Debug, Serialize)]
pub struct GetModelReq<'a> {
  repo_id: &'a str,
  revision: Option<&'a str>,
}

/// Response of [`crate::client::Client::get_model`]
pub type GetModelRes = Model;
