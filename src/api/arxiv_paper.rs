use serde::{Deserialize, Serialize};

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
#[derive(Debug, Deserialize)]
pub struct ArxivPaperRes {
  id: String,
  authors: Vec<ArxivPaperAuthor>,

  #[serde(rename = "publishedAt")]
  published_time: String,

  #[serde(rename = "submittedOnDailyAt")]
  submitted_time: String,

  title: String,

  submitted_by: Option<ArxivPaperUser>,

  summary: Option<String>,

  #[serde(rename = "upvotes")]
  up_votes: usize,

  discussion_id: Option<String>,

  ai_keywords: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct ArxivPaperAuthor {
  #[serde(rename = "_id")]
  id: String,
  user: Option<ArxivPaperUser>,
  name: String,
  status: Option<String>,
  hidden: bool,
}

#[derive(Debug, Deserialize)]
pub struct ArxivPaperUser {
  #[serde(rename = "_id")]
  id: String,

  #[serde(rename = "avatarUrl")]
  avatar_url: Option<String>,

  #[serde(rename = "fullname")]
  full_name: String,

  user: String,
}
