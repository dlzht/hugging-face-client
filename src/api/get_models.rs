use serde::Serialize;

use crate::{model::Model, tag::ModelTag};

/// Request of [`crate::client::Client::get_models`]
#[derive(Debug, Default, Serialize)]
pub struct GetModelsReq<'a> {
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

impl<'a> GetModelsReq<'a> {
  pub fn search(mut self, search: Option<&'a str>) -> Self {
    self.search = search;
    self
  }

  pub fn author(mut self, author: Option<&'a str>) -> Self {
    self.author = author;
    self
  }

  pub fn filter(mut self, filter: Option<&'a str>) -> Self {
    self.filter = filter;
    self
  }

  pub fn sort(mut self, sort: Option<&'a str>) -> Self {
    self.sort = sort;
    self
  }

  pub fn direction(mut self, direction: Option<i32>) -> Self {
    self.direction = direction;
    self
  }

  pub fn limit(mut self, limit: Option<usize>) -> Self {
    self.limit = limit;
    self
  }

  pub fn full(mut self, full: Option<bool>) -> Self {
    self.full = full;
    self
  }

  pub fn config(mut self, config: Option<bool>) -> Self {
    self.config = config;
    self
  }
}

/// Response of [`crate::client::Client::get_models`]
pub type GetModelsRes = Vec<Model>;

#[cfg(test)]
mod test {
  use std::assert_matches::assert_matches;

  use crate::api::GetModelsReq;

  #[test]
  fn test_serde_req() {
    let req: GetModelsReq = GetModelsReq::default()
      .search(Some("1"))
      .author(Some("2"))
      .filter(Some("3"))
      .sort(Some("4"))
      .direction(Some(1))
      .limit(Some(100))
      .full(Some(true))
      .config(Some(true));
    let query = serde_urlencoded::to_string(req);
    assert_matches!(query, Ok(v) if v == "search=1&author=2&filter=3&sort=4&direction=1&limit=100&full=true&config=true");
  }
}
