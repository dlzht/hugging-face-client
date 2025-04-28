use serde::Deserialize;

use crate::tag::DatasetTag;

/// Response of [`crate::client::Client::get_dataset_tags`]
#[derive(Debug, Deserialize)]
pub struct GetDatasetTagRes {
  pub region: Vec<DatasetTag>,
  pub library: Vec<DatasetTag>,
  pub license: Vec<DatasetTag>,
  pub language: Vec<DatasetTag>,
  pub dataset: Vec<DatasetTag>,

  #[serde(rename = "pipeline_tag")]
  pub pipeline: Vec<DatasetTag>,

  pub other: Vec<DatasetTag>,
}
