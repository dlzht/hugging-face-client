use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dataset {
  #[serde(rename = "_id")]
  id: String,

  #[serde(rename = "id")]
  name: String,

  author: String,

  disabled: bool,

  #[serde(rename = "trendingScore")]
  trending_source: Option<usize>,

  private: bool,
  likes: usize,
  downloads: usize,
  pipeline_tag: Option<String>,
  library_name: Option<String>,

  #[serde(rename = "createdAt")]
  created_time: String,
  tags: Vec<String>,

  gated: Option<bool>,

  #[serde(rename = "lastModified")]
  last_modified: Option<String>,

  sha: Option<String>,

  description: Option<String>,

  key: Option<String>,

  #[serde(rename = "cardData")]
  card_data: Option<CardData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardData {
  license: Option<String>,
  tags: Option<Vec<String>>,
  task_categories: Option<Vec<String>>,
  size_categories: Option<Vec<String>>,
}
