use serde::Serialize;

use crate::collection::Collection;

/// Request of [`crate::client::Client::get_collection`]
#[derive(Debug, Serialize)]
pub struct GetCollectionReq {
  // pub(crate) namespace: &'a str,
  pub(crate) slug: String,
  // pub(crate) id: &'a str,
}

impl GetCollectionReq {
  // pub fn new(namespace: &'a str, slug: &'a str, id: &'a str) -> Self {
  //   GetCollectionReq {
  //     namespace,
  //     slug,
  //     id
  //   }
  // }

  pub fn new(slug: impl Into<String>) -> Self {
    GetCollectionReq { slug: slug.into() }
  }
}

impl<T: Into<String>> From<T> for GetCollectionReq {
  fn from(s: T) -> Self {
    Self { slug: s.into() }
  }
}

/// Response of [`crate::client::Client::get_collection`],
/// [`crate::client::Client::create_collection`]
pub type GetCollectionRes = Collection;
