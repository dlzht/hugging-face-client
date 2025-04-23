use serde::Serialize;

use crate::{api::SearchReq, model::Model};

/// Request of [`crate::client::Client::get_models`]
pub type GetModelsReq<'a> = SearchReq<'a>;

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
