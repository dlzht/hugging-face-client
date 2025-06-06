use crate::{api::SearchReq, space::Space};

/// Request of [`crate::client::Client::search_space`]
pub type SearchSpaceReq<'a> = SearchReq<'a>;

/// Response of [`crate::client::Client::search_space`]
pub type SearchSpaceRes = Vec<Space>;

#[cfg(test)]
mod test {
  use std::assert_matches::assert_matches;

  use crate::api::SearchSpaceReq;

  #[test]
  fn test_serde_req() {
    let req = SearchSpaceReq::default()
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
