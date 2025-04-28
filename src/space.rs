use serde::{Deserialize, Serialize};

use crate::{dataset::DatasetCardData, sibling::Sibling};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Space {
  #[serde(rename = "_id")]
  pub id: String,

  #[serde(rename = "id")]
  pub name: String,

  #[serde(rename = "trendingScore")]
  pub trending_source: Option<usize>,

  pub private: bool,
  pub likes: usize,

  #[serde(rename = "createdAt")]
  pub created_time: String,

  pub tags: Vec<String>,
  pub sdk: Option<SpaceSdkType>,
  pub sha: Option<String>,
  pub subdomain: Option<String>,

  #[serde(rename = "lastModified")]
  pub last_modified: Option<String>,

  pub siblings: Option<Vec<Sibling>>,

  #[serde(rename = "cardData")]
  pub card_data: Option<DatasetCardData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceCardData {
  pub title: Option<String>,
  pub emoji: Option<String>,
  pub sdk_version: Option<String>,
  pub app_file: Option<String>,
  pub license: Option<String>,
  pub short_description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpaceSdkType {
  #[serde(rename = "streamlit")]
  StreamLit,

  #[serde(rename = "gradio")]
  Gradio,

  #[serde(rename = "docker")]
  Docker,

  #[serde(rename = "static")]
  Static,

  Unknown(String),
}

impl Default for SpaceSdkType {
  fn default() -> Self {
    SpaceSdkType::Static
  }
}

#[cfg(test)]
mod test {
  use std::assert_matches::assert_matches;

  use crate::space::SpaceSdkType;

  #[test]
  fn test_serde_sdk_type() {
    let sdk = SpaceSdkType::StreamLit;
    let json = serde_json::to_string(&sdk);
    assert_matches!(json, Ok(v) if v == "\"streamlit\"");

    let json = "\"streamlit\"";
    let sdk = serde_json::from_str::<SpaceSdkType>(json);
    assert_matches!(sdk, Ok(SpaceSdkType::StreamLit));

    let sdk = SpaceSdkType::Gradio;
    let json = serde_json::to_string(&sdk);
    assert_matches!(json, Ok(v) if v == "\"gradio\"");

    let json = "\"gradio\"";
    let sdk = serde_json::from_str::<SpaceSdkType>(json);
    assert_matches!(sdk, Ok(SpaceSdkType::Gradio));

    let sdk = SpaceSdkType::Docker;
    let json = serde_json::to_string(&sdk);
    assert_matches!(json, Ok(v) if v == "\"docker\"");

    let json = "\"docker\"";
    let sdk = serde_json::from_str::<SpaceSdkType>(json);
    assert_matches!(sdk, Ok(SpaceSdkType::Docker));

    let sdk = SpaceSdkType::Static;
    let json = serde_json::to_string(&sdk);
    assert_matches!(json, Ok(v) if v == "\"static\"");

    let json = "\"static\"";
    let sdk = serde_json::from_str::<SpaceSdkType>(json);
    assert_matches!(sdk, Ok(SpaceSdkType::Static));
  }

  #[test]
  fn test_sdk_type_default() {
    let sdk = SpaceSdkType::default();
    assert_eq!(sdk, SpaceSdkType::Static);
  }
}
