use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
  pub id: String,

  #[serde(rename = "spaceId")]
  pub space_id: String,

  pub description: Option<String>,
}
