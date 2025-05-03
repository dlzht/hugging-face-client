//! ### Create Client
//!
//! ```rust
//! use std::time::Duration;
//! use hugging_face_client::client::{Client, ClientOption};
//!
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() {
//!   let access_token = std::env::var("HUGGING_FACE_TOKEN").unwrap();
//!   let access_proxy = std::env::var("HUGGING_FACE_PROXY").unwrap();
//!   let option = ClientOption::new(access_token)
//!     .proxy(access_proxy)
//!     .timeout(Duration::from_secs(5));
//!   let client = Client::new(option).unwrap();
//! }
//! ```
//!
//! ### Get Model
//! ```rust
//! use hugging_face_client::api::GetModelReq;
//! use hugging_face_client::client::{Client, ClientOption};
//!
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() {
//!   let option = ClientOption::new("HUGGING_FACE_TOKEN");
//!   let client = Client::new(option).unwrap();
//!
//!   // get model
//!   let req = GetModelReq::new("microsoft/bitnet-b1.58-2B-4T");
//!   let res = client.get_model(req).await.unwrap();
//!   println!("{:#?}", res);
//! }
//! // Model { _id: "67fddfa9a7fe1f21ec1d3026", id: "microsoft/bitnet-b1.58-2B-4T", model_id: None ... }
//! ```
//! **More usage examples, can be seen [hugging-face-client/examples](https://github.com/dlzht/hugging-face-client/tree/main/examples)**

#![feature(assert_matches)]

pub mod api;
pub mod client;
pub mod errors;

mod arxiv;
mod collection;
mod config;
mod dataset;
mod metric;
mod model;
mod paginamtion;
mod provider;
mod repo;
mod runtime;
mod sibling;
mod sort;
mod space;
mod tag;
mod user;

pub use repo::RepoType;
