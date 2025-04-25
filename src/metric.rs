use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
  id: String,

  #[serde(rename = "spaceId")]
  space_id: String,

  description: Option<String>,
}

impl Metric {
  pub fn id(&self) -> &str {
    &self.id
  }

  pub fn space_id(&self) -> &str {
    &self.space_id
  }

  pub fn description(&self) -> Option<&str> {
    self.description.as_deref()
  }
}
