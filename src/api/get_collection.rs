use serde::Serialize;

use crate::collection::Collection;

/// Request of [`crate::client::Client::get_collection`]
#[derive(Debug, Serialize)]
pub struct GetCollectionReq<'a> {
  // pub(crate) namespace: &'a str,
  pub(crate) slug: &'a str,
  // pub(crate) id: &'a str,
}

impl<'a> GetCollectionReq<'a> {
  // pub fn new(namespace: &'a str, slug: &'a str, id: &'a str) -> Self {
  //   GetCollectionReq {
  //     namespace,
  //     slug,
  //     id
  //   }
  // }

  pub fn new(slug: &'a str) -> Self {
    GetCollectionReq { slug }
  }
}

/// Response of [`crate::client::Client::get_collection`],
/// [`crate::client::Client::create_collection`]
pub type GetCollectionRes = Collection;
