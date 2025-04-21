//! Hugging Face Hub API
//!
//! Hugging Face has open endpoints that you can use to retrieve information from the Hub as well as perform
//! certain actions such as creating model, dataset or Space repos.

mod get_models;
pub use get_models::{GetModelsReq, GetModelsRes};

mod get_model;
pub use get_model::{GetModelReq, GetModelRes};
