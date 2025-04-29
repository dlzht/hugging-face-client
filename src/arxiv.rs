use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ArxivPaper {
  pub id: String,
  pub authors: Vec<ArxivAuthor>,

  #[serde(rename = "publishedAt")]
  pub published_time: String,

  #[serde(rename = "submittedOnDailyAt")]
  pub submitted_time: String,

  pub title: String,
  pub submitted_by: Option<ArxivUser>,
  pub summary: Option<String>,

  #[serde(rename = "upvotes")]
  pub up_votes: usize,

  pub discussion_id: Option<String>,
  pub ai_keywords: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArxivAuthor {
  #[serde(rename = "_id")]
  pub id: String,

  pub user: Option<ArxivUser>,
  pub name: String,
  pub status: Option<String>,
  pub hidden: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArxivUser {
  #[serde(rename = "_id")]
  pub id: String,

  #[serde(rename = "avatarUrl")]
  pub avatar_url: Option<String>,

  #[serde(rename = "fullname")]
  pub full_name: String,

  pub user: String,
}
