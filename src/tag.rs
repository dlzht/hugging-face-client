use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Tag {
  id: String,
  label: String,
  tag_type: Option<TagType>,
  sub_type: Option<String>,
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
