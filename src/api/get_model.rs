use serde::Serialize;

use crate::model::Model;

/// Request of `GET /api/models/{repo_id}` or `/api/models/{repo_id}/revision/{revision}`
#[derive(Debug, Serialize)]
pub struct GetModelReq<'a> {
  repo_id: &'a str,
  revision: Option<&'a str>,
}

/// Response of `GET /api/models/{repo_id}` or `/api/models/{repo_id}/revision/{revision}`
pub type GetModelRes = Model;
