use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
  pub id: String,
  pub label: String,
  pub tag_type: Option<TagType>,
  pub sub_type: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TagType {
  #[serde(rename = "region")]
  Region,

  #[serde(rename = "library")]
  Library,

  #[serde(rename = "license")]
  License,

  #[serde(rename = "language")]
  Language,

  #[serde(rename = "dataset")]
  Dataset,

  #[serde(rename = "pipeline_tag")]
  Pipeline,

  #[serde(rename = "other")]
  Other,

  Unknown(String),
}

pub type ModelTag = Tag;
pub type DatasetTag = Tag;
