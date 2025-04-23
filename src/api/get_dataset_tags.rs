use serde::Deserialize;

use crate::tag::ModelTag;

/// Response of [`crate::client::Client::get_dataset_tags`]
#[derive(Debug, Deserialize)]
pub struct GetDatasetTagRes {
  region: Vec<ModelTag>,
  library: Vec<ModelTag>,
  license: Vec<ModelTag>,
  language: Vec<ModelTag>,
  dataset: Vec<ModelTag>,

  #[serde(rename = "pipeline_tag")]
  pipeline: Vec<ModelTag>,

  other: Vec<ModelTag>,
}

impl GetDatasetTagRes {
  pub fn region(&self) -> &[ModelTag] {
    &self.region
  }

  pub fn library(&self) -> &[ModelTag] {
    &self.library
  }

  pub fn license(&mut self) -> &[ModelTag] {
    &self.license
  }

  pub fn language(&self) -> &[ModelTag] {
    &self.language
  }

  pub fn dataset(&self) -> &[ModelTag] {
    &self.dataset
  }

  pub fn pipeline(&self) -> &[ModelTag] {
    &self.pipeline
  }

  pub fn other(&self) -> &[ModelTag] {
    &self.other
  }
}
