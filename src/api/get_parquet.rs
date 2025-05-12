use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Request of [`crate::client::Client::get_parquet`]
#[derive(Debug, Serialize)]
pub struct GetParquetReq<'a> {
  #[serde(rename = "repo_id")]
  pub(crate) repo_name: &'a str,

  pub(crate) subset: Option<&'a str>,

  pub(crate) split: Option<&'a str>,
}

impl<'a> GetParquetReq<'a> {
  pub fn new(repo_name: &'a str) -> GetParquetReq<'a> {
    GetParquetReq {
      repo_name,
      subset: None,
      split: None,
    }
  }

  pub fn subset(mut self, subset: &'a str) -> Self {
    self.subset = Some(subset);
    self
  }

  pub fn split(mut self, split: &'a str) -> Self {
    self.split = Some(split);
    self
  }
}

/// Response of [`crate::client::Client::get_parquet`]
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum GetParquetRes {
  Repo(HashMap<String, HashMap<String, Vec<String>>>),
  Subset(HashMap<String, Vec<String>>),
  Split(Vec<String>),
}
