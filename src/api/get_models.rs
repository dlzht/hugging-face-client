use serde::Serialize;

use crate::model::Model;

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

/// Response of [`crate::client::Client::get_models`]
pub type GetModelsRes = Vec<Model>;

#[cfg(test)]
mod test {
  use std::assert_matches::assert_matches;

  use crate::api::GetModelsReq;

  #[test]
  fn test_serde_req() {
    let req: GetModelsReq = GetModelsReq::default()
      .search("1")
      .author("2")
      .filter("3")
      .sort("4")
      .direction(1)
      .limit(100)
      .full(true)
      .config(true);
    let query = serde_urlencoded::to_string(req);
    assert_matches!(query, Ok(v) if v == "search=1&author=2&filter=3&sort=4&direction=1&limit=100&full=true&config=true");
  }
}
