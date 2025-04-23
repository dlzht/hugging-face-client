use serde::Deserialize;

use crate::tag::Tag;

#[derive(Debug, Deserialize)]
pub struct GetTagsRes {
  region: Vec<Tag>,
  library: Vec<Tag>,
  license: Vec<Tag>,
  language: Vec<Tag>,
  dataset: Vec<Tag>,

  #[serde(rename = "pipeline_tag")]
  pipeline: Vec<Tag>,

  other: Vec<Tag>,
}

impl GetTagsRes {
  pub fn region(&self) -> &[Tag] {
    &self.region
  }

  pub fn library(&self) -> &[Tag] {
    &self.library
  }

  pub fn license(&mut self) -> &[Tag] {
    &self.license
  }

  pub fn language(&self) -> &[Tag] {
    &self.language
  }

  pub fn dataset(&self) -> &[Tag] {
    &self.dataset
  }

  pub fn pipeline(&self) -> &[Tag] {
    &self.pipeline
  }

  pub fn other(&self) -> &[Tag] {
    &self.other
  }
}
