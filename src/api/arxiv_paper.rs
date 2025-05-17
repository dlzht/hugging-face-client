use serde::Serialize;

use crate::arxiv::ArxivPaper;

/// Request of [`crate::client::Client::arxiv_paper`]
#[derive(Debug, Serialize)]
pub struct ArxivPaperReq {
  pub(crate) paper_id: String,
}

impl<'a> ArxivPaperReq {
  pub fn new(paper_id: impl Into<String>) -> Self {
    Self { paper_id: paper_id.into() }
  }
}

impl<T: Into<String>> From<T> for ArxivPaperReq {
  fn from(paper_id: T) -> Self {
    Self {
      paper_id: paper_id.into(),
    }
  }
}

/// Request of [`crate::client::Client::arxiv_paper`]
pub type ArxivPaperRes = ArxivPaper;
