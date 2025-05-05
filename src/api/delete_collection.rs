use serde::{Deserialize, Serialize};

/// Request of [`crate::client::Client::delete_collection`]
#[derive(Debug, Serialize)]
pub struct DeleteCollectionReq<'a> {
  pub(crate) slug: &'a str,
}

impl<'a> DeleteCollectionReq<'a> {
  pub fn new(slug: &'a str) -> DeleteCollectionReq<'a> {
    DeleteCollectionReq { slug }
  }
}

/// Response of [`crate::client::Client::delete_collection`]
#[derive(Debug, Deserialize)]
pub struct DeleteCollectionRes {
  pub success: bool,
}
