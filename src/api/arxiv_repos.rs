use serde::{Deserialize, Serialize};

use crate::repo::Repo;

/// Request of [`crate::client::Client::arxiv_repos`]
#[derive(Debug, Serialize)]
pub struct ArxivReposReq {
  pub(crate) paper_id: String,
}

impl ArxivReposReq {
  pub fn new(paper_id: impl Into<String>) -> Self {
    ArxivReposReq {
      paper_id: paper_id.into(),
    }
  }
}

impl<T: Into<String>> From<T> for ArxivReposReq {
  fn from(paper_id: T) -> Self {
    Self {
      paper_id: paper_id.into(),
    }
  }
}

/// Response of [`crate::client::Client::arxiv_repos`]
#[derive(Debug, Deserialize)]
pub struct ArxivReposRes {
  pub models: Vec<Repo>,
}
