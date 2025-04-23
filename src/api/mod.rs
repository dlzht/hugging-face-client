//! Request and response
//!
//! Hugging Face has open endpoints that you can use to retrieve information from the Hub as well as perform
//! certain actions such as creating model, dataset or Space repos.

use serde::Deserialize;

use crate::errors::{HuggingFaceResponseSnafu, PlainMessageSnafu, Result};

mod get_models;
pub use get_models::{GetModelsReq, GetModelsRes};

mod get_model;
pub use get_model::{GetModelReq, GetModelRes};

mod create_repo;
pub use create_repo::{CreateRepoReq, CreateRepoRes};

mod delete_repo;
pub use delete_repo::DeleteRepoReq;

mod get_tags;
pub use get_tags::GetTagsRes;

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
