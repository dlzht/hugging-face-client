use serde::{Deserialize, Serialize};

pub struct Repository {

}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RepoType {

  #[serde(rename = "dataset")]
  Dataset,

  #[serde(rename = "space")]
  Space,

  #[serde(rename = "model")]
  Model
}

impl Default for RepoType {
  fn default() -> Self {
    RepoType::Model
  }
}