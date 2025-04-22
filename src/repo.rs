use serde::{Deserialize, Serialize};

pub struct Repository {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RepoType {
  #[serde(rename = "dataset")]
  Dataset,

  #[serde(rename = "space")]
  Space,

  #[serde(rename = "model")]
  Model,
}

impl Default for RepoType {
  fn default() -> Self {
    RepoType::Model
  }
}

#[cfg(test)]
mod test {
  use std::assert_matches::assert_matches;

  use crate::repo::RepoType;

  #[test]
  fn test_serde_repo_type() {
    let repo_type = RepoType::Dataset;
    let json = serde_json::to_string(&repo_type);
    assert_matches!(json, Ok(v) if v == "\"dataset\"");

    let json = "\"dataset\"";
    let sdk = serde_json::from_str::<RepoType>(json);
    assert_matches!(sdk, Ok(RepoType::Dataset));

    let repo_type = RepoType::Space;
    let json = serde_json::to_string(&repo_type);
    assert_matches!(json, Ok(v) if v == "\"space\"");

    let json = "\"space\"";
    let sdk = serde_json::from_str::<RepoType>(json);
    assert_matches!(sdk, Ok(RepoType::Space));

    let repo_type = RepoType::Model;
    let json = serde_json::to_string(&repo_type);
    assert_matches!(json, Ok(v) if v == "\"model\"");

    let json = "\"model\"";
    let sdk = serde_json::from_str::<RepoType>(json);
    assert_matches!(sdk, Ok(RepoType::Model));
  }
}
