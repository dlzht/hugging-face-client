use serde::{Deserialize, Serialize};

use crate::repo::Repo;

/// Request of [`crate::client::Client::arxiv_repos`]
#[derive(Debug, Serialize)]
pub struct ArxivReposReq<'a> {
  pub(crate) paper_id: &'a str,
}

impl<'a> ArxivReposReq<'a> {
  pub fn new(arxiv_id: &'a str) -> Self {
    ArxivReposReq { paper_id: arxiv_id }
  }
}

/// Response of [`crate::client::Client::arxiv_repos`]
#[derive(Debug, Deserialize)]
pub struct ArxivReposRes {
  pub models: Vec<Repo>,
}
