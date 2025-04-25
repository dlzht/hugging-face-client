//! Request and response
//!
//! Hugging Face has open endpoints that you can use to retrieve information from the Hub as well as perform
//! certain actions such as creating model, dataset or Space repos.

use serde::{Deserialize, Serialize};

use crate::errors::{HuggingFaceResponseSnafu, Result};

mod search_models;
pub use search_models::{SearchModelReq, SearchModelRes};

mod get_model;
pub use get_model::{GetModelReq, GetModelRes};

mod create_repo;
pub use create_repo::{CreateRepoReq, CreateRepoRes};

mod delete_repo;
pub use delete_repo::DeleteRepoReq;

mod get_model_tags;
pub use get_model_tags::GetModelTagsRes;

mod search_datasets;
pub use search_datasets::{SearchDatasetReq, SearchDatasetRes};

mod get_dataset;
pub use get_dataset::{GetDatasetReq, GetDatasetRes};

mod get_dataset_tags;
pub use get_dataset_tags::GetDatasetTagRes;

mod get_space;
pub use get_space::{GetSpaceReq, GetSpaceRes};

mod search_spaces;
pub use search_spaces::{SearchSpaceReq, SearchSpaceRes};

mod move_repo;
pub use move_repo::MoveRepoReq;

mod get_metrics;
pub use get_metrics::GetMetricsRes;

/// Global response format of Hugging Face Hub API
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum HuggingFaceRes<T> {
  Success(T),
  Failure(FailureRes),
}

impl<T> HuggingFaceRes<T> {
  /// Unwrap data from [`HuggingFaceRes::Success`]
  pub fn unwrap_data(self) -> Result<T> {
    match self {
      HuggingFaceRes::Success(v) => Ok(v),
      HuggingFaceRes::Failure(f) => Err(
        HuggingFaceResponseSnafu {
          message: f.error().to_string(),
        }
        .build(),
      ),
    }
  }
}

/// Error response format of Hugging Face Hub API
#[derive(Debug, Clone, Deserialize)]
pub struct FailureRes {
  error: String,
}

impl FailureRes {
  /// Get error message
  pub fn error(&self) -> &str {
    &self.error
  }
}

#[derive(Debug, Default, Serialize)]
pub struct SearchReq<'a> {
  #[serde(skip_serializing_if = "Option::is_none")]
  search: Option<&'a str>,

  #[serde(skip_serializing_if = "Option::is_none")]
  author: Option<&'a str>,

  #[serde(skip_serializing_if = "Option::is_none")]
  filter: Option<&'a str>,

  #[serde(skip_serializing_if = "Option::is_none")]
  sort: Option<&'a str>,

  #[serde(skip_serializing_if = "Option::is_none")]
  direction: Option<i32>,

  #[serde(skip_serializing_if = "Option::is_none")]
  limit: Option<usize>,

  #[serde(skip_serializing_if = "Option::is_none")]
  full: Option<bool>,

  #[serde(skip_serializing_if = "Option::is_none")]
  config: Option<bool>,
}

impl<'a> SearchReq<'a> {
  pub fn search(mut self, search: &'a str) -> Self {
    self.search = Some(search);
    self
  }

  pub fn author(mut self, author: &'a str) -> Self {
    self.author = Some(author);
    self
  }

  pub fn filter(mut self, filter: &'a str) -> Self {
    self.filter = Some(filter);
    self
  }

  pub fn sort(mut self, sort: &'a str) -> Self {
    self.sort = Some(sort);
    self
  }

  pub fn direction(mut self, direction: i32) -> Self {
    self.direction = Some(direction);
    self
  }

  pub fn limit(mut self, limit: usize) -> Self {
    self.limit = Some(limit);
    self
  }

  pub fn full(mut self, full: bool) -> Self {
    self.full = Some(full);
    self
  }

  pub fn config(mut self, config: bool) -> Self {
    self.config = Some(config);
    self
  }
}
