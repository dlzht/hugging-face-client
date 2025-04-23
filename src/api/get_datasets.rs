use crate::{api::SearchReq, dataset::Dataset};

/// Request of [`crate::client::Client::get_datasets`]
pub type GetDatasetsReq<'a> = SearchReq<'a>;

/// Response of [`crate::client::Client::get_datasets`]
pub type GetDatasetsRes = Vec<Dataset>;
