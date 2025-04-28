use serde::Deserialize;

use crate::tag::ModelTag;

/// Response of [`crate::client::Client::get_model_tags`]
#[derive(Debug, Deserialize)]
pub struct GetModelTagsRes {
  pub region: Vec<ModelTag>,
  pub library: Vec<ModelTag>,
  pub license: Vec<ModelTag>,
  pub language: Vec<ModelTag>,
  pub dataset: Vec<ModelTag>,

  #[serde(rename = "pipeline_tag")]
  pub pipeline: Vec<ModelTag>,

  pub other: Vec<ModelTag>,
}
