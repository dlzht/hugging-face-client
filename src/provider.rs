use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceProvider {
  pub provider: String,
  pub model_status: String,
  pub provide_status: String,
  pub provider_id: String,
  pub task: String,

  #[serde(rename = "adapterWeightsPath")]
  pub adapter_weights_path: String,
}
