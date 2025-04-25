use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

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

impl Display for RepoType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      RepoType::Dataset => f.write_str("dataset"),
      RepoType::Space => f.write_str("space"),
      RepoType::Model => f.write_str("model"),
    }
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
