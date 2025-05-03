use serde::Serialize;

use crate::RepoType;

#[derive(Debug, Serialize)]
pub struct CreateCollectionItemReq<'a> {
  item_type: RepoType,
  id: &'a str,
}
