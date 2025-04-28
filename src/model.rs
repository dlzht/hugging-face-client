use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
  #[serde(rename = "_id")]
  pub id: String,

  #[serde(rename = "id")]
  pub name: String,

  pub model_id: Option<String>,

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
}
