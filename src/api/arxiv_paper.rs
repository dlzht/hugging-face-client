use serde::Serialize;

use crate::arxiv::ArxivPaper;

/// Request of [`crate::client::Client::arxiv_paper`]
#[derive(Debug, Serialize)]
pub struct ArxivPaperReq<'a> {
  pub(crate) paper_id: &'a str,
}

impl<'a> ArxivPaperReq<'a> {
  pub fn new(paper_id: &'a str) -> Self {
    Self { paper_id }
  }
}

/// Request of [`crate::client::Client::arxiv_paper`]
pub type ArxivPaperRes = ArxivPaper;
