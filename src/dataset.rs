use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dataset {
  #[serde(rename = "_id")]
  pub id: String,

  #[serde(rename = "id")]
  pub name: String,

  pub author: String,
  pub disabled: bool,

  #[serde(rename = "trendingScore")]
  pub trending_source: Option<usize>,

  pub private: bool,
  pub likes: usize,
  pub downloads: usize,
  pub pipeline_tag: Option<String>,
  pub library_name: Option<String>,

  #[serde(rename = "createdAt")]
  pub created_time: String,

  pub tags: Vec<String>,
  pub gated: Option<bool>,

  #[serde(rename = "lastModified")]
  pub last_modified: Option<String>,

  pub sha: Option<String>,
  pub description: Option<String>,
  pub key: Option<String>,

  #[serde(rename = "cardData")]
  pub card_data: Option<DatasetCardData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetCardData {
  pub license: Option<String>,
  pub tags: Option<Vec<String>>,
  pub task_categories: Option<Vec<String>>,
  pub size_categories: Option<Vec<String>>,
}
