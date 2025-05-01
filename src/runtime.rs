use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Runtime {
  stage: String,
  hardware: RuntimeHardware,
  storage: Option<String>,

  #[serde(rename = "gcTimeout")]
  gc_timeout: usize,

  #[serde(rename = "devMode")]
  dev_mode: bool,

  domains: Vec<RuntimeDomain>,

  sha: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeHardware {
  current: String,

  requested: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeReplicas {
  current: usize,
  requested: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeDomain {
  domain: String,
  stage: String,
}
