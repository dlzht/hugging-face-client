use serde::Deserialize;

use crate::tag::ModelTag;

#[derive(Debug, Deserialize)]
pub struct GetTagsRes {
  region: Vec<ModelTag>,
  library: Vec<ModelTag>,
  license: Vec<ModelTag>,
  language: Vec<ModelTag>,
  dataset: Vec<ModelTag>,

  #[serde(rename = "pipeline_tag")]
  pipeline: Vec<ModelTag>,

  other: Vec<ModelTag>,
}

impl GetTagsRes {
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
