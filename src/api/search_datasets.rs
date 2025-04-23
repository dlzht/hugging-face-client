use crate::{api::SearchReq, dataset::Dataset};

/// Request of [`crate::client::Client::search_dataset`]
pub type GetDatasetsReq<'a> = SearchReq<'a>;

/// Response of [`crate::client::Client::search_dataset`]
pub type GetDatasetsRes = Vec<Dataset>;
