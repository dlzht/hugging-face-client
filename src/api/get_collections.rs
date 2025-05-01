use serde::Serialize;

use crate::collection::Collection;

/// Request of [`crate::client::Client::get_collections`]
#[derive(Debug, Serialize)]
pub struct GetCollectionsReq<'a> {
  #[serde(skip_serializing_if = "Option::is_none")]
  owner: Option<&'a str>,

  #[serde(skip_serializing_if = "Option::is_none")]
  item: Option<&'a str>,

  #[serde(skip_serializing_if = "Option::is_none")]
  sort: Option<&'a str>,

  #[serde(skip_serializing_if = "Option::is_none")]
  limit: Option<usize>,

  #[serde(skip_serializing_if = "Option::is_none", rename = "q")]
  query: Option<&'a str>,
}

impl<'a> GetCollectionsReq<'a> {
  pub fn new() -> Self {
    GetCollectionsReq {
      owner: None,
      item: None,
      sort: None,
      limit: None,
      query: None,
    }
  }

  /// Filter collections created by a specific user or organization
  pub fn owner(mut self, owner: &'a str) -> Self {
    self.owner = Some(owner);
    self
  }

  /// Filter collections containing a specific item. Value must be the item_type and item_id
  /// concatenated. Example: "models/teknium/OpenHermes-2.5-Mistral-7B", "datasets/rajpurkar/squad"
  /// or "papers/2311.12983"
  pub fn item(mut self, item: &'a str) -> Self {
    self.item = Some(item);
    self
  }

  /// Sort the returned collections. Supported values are "lastModified", "trending" (default) and
  /// "upvotes"
  pub fn sort(mut self, sort: &'a str) -> Self {
    self.sort = Some(sort);
    self
  }

  /// Maximum number (100) of collections per page
  pub fn limit(mut self, limit: usize) -> Self {
    self.limit = Some(limit);
    self
  }

  ///  Filter based on substrings for titles & descriptions
  pub fn query(mut self, query: &'a str) -> Self {
    self.query = Some(query);
    self
  }
}

/// Response of [`crate::client::Client::get_collections`]
pub type GetCollectionsRes = Vec<Collection>;
