use crate::{api::SearchReq, dataset::Dataset};

/// Request of [`crate::client::Client::search_dataset`]
pub type SearchDatasetReq<'a> = SearchReq<'a>;

/// Response of [`crate::client::Client::search_dataset`]
pub type SearchDatasetRes = Vec<Dataset>;
