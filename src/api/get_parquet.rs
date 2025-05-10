use serde::{Deserialize, Serialize};

/// Request of [`crate::client::Client::get_parquet`]
#[derive(Debug, Serialize)]
pub struct GetParquetReq<'a> {
  #[serde(rename = "repo_id")]
  pub(crate) repo_name: &'a str,
}

impl<'a> GetParquetReq<'a> {
  pub fn new(repo_name: &str) -> GetParquetReq<'_> {
    GetParquetReq { repo_name }
  }
}

/// Response of [`crate::client::Client::get_parquet`]
#[derive(Debug, Deserialize)]
pub struct GetParquetRes {
  pub default: GetParquetTrain,
}

#[derive(Debug, Deserialize)]
pub struct GetParquetTrain {
  pub train: Vec<String>,
}
