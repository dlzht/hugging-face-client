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
  pub id: String,
  pub authors: Vec<ArxivPaperAuthor>,

  #[serde(rename = "publishedAt")]
  pub published_time: String,

  #[serde(rename = "submittedOnDailyAt")]
  pub submitted_time: String,

  pub title: String,
  pub submitted_by: Option<ArxivPaperUser>,
  pub summary: Option<String>,

  #[serde(rename = "upvotes")]
  pub up_votes: usize,

  pub discussion_id: Option<String>,
  pub ai_keywords: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct ArxivPaperAuthor {
  #[serde(rename = "_id")]
  pub id: String,

  pub user: Option<ArxivPaperUser>,
  pub name: String,
  pub status: Option<String>,
  pub hidden: bool,
}

#[derive(Debug, Deserialize)]
pub struct ArxivPaperUser {
  #[serde(rename = "_id")]
  pub id: String,

  #[serde(rename = "avatarUrl")]
  pub avatar_url: Option<String>,

  #[serde(rename = "fullname")]
  pub full_name: String,

  pub user: String,
}
