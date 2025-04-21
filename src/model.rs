use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
  _id: String,
  id: String,
  model_id: String,

  #[serde(rename = "trendingScore")]
  trending_source: usize,

  private: bool,
  likes: usize,
  downloads: usize,
  pipeline_tag: String,
  library_name: String,

  #[serde(rename = "createdAt")]
  created_time: String,
  tags: Vec<String>,

  gated: Option<bool>,

  #[serde(rename = "lastModified")]
  last_modified: Option<String>,

  sha: Option<String>,
}
