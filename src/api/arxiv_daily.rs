use serde::Deserialize;

use crate::arxiv::ArxivPaper;

/// Request of [`crate::client::Client::arxiv_daily`]
pub type ArxivDailyRes = Vec<ArxivDailyItem>;

#[derive(Debug, Deserialize)]
pub struct ArxivDailyItem {
  pub paper: ArxivPaper,

  #[serde(rename = "publishedAt")]
  pub published_time: String,

  pub title: String,

  pub summary: String,

  pub thumbnail: String,

  #[serde(rename = "num_comments")]
  pub comment_count: Option<usize>,

  #[serde(rename = "submittedBy")]
  pub submitter: ArxivDailySubmitter,

  #[serde(rename = "isAuthorParticipating")]
  pub is_author_participating: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ArxivDailySubmitter {
  #[serde(rename = "_id")]
  pub id: String,

  #[serde(rename = "avatarUrl")]
  pub avatar_url: Option<String>,

  #[serde(rename = "fullname")]
  pub full_name: String,

  pub name: String,

  pub follower_count: Option<usize>,
}
