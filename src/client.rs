//! Async hub client

use reqwest::Client as ReqwestClient;

use crate::api::{
  CreateRepoReq, CreateRepoRes, GetModelReq, GetModelRes, GetModelsReq, GetModelsRes,
};

const DEFAULT_API_ENDPOINT: &'static str = "https://huggingface.co";

/// Async client for Hugging Face Hub
#[derive(Debug, Clone)]
pub struct Client {
  access_token: String,
  api_endpoint: String,
  http_client: ReqwestClient,
}

impl Client {
  /// Create Hugging Face Hub client with default api endpoint
  ///
  /// `access_token`: authenticate client to the Hugging Face Hub and allow client to perform
  /// actions based on token permissions, see [https://huggingface.co/settings/tokens](https://huggingface.co/settings/tokens)
  ///
  /// ```rust
  /// use hugging_face_client::client::Client;
  /// fn main() {
  ///   let client = Client::new("hf-Hugging-Face-Access-Token");
  /// }
  /// ```
  pub fn new(access_token: impl Into<String>) -> Self {
    Client {
      access_token: access_token.into(),
      api_endpoint: DEFAULT_API_ENDPOINT.to_string(),
      http_client: ReqwestClient::new(),
    }
  }

  /// Set the api endpoint, it's useful if you want to access hub thought reverse proxy
  ///
  /// `endpoint`: endpoint of Hugging Face Hub api, and final urls will be concatenated in the
  /// format of `{endpoint}/api/models`
  ///
  /// ```rust
  /// use hugging_face_client::client::Client;
  /// fn main() {
  ///   let mut client = Client::new("hf-Hugging-Face-Access-Token");
  ///   client.endpoint("https://proxy-to-hugging-face.com");
  /// }
  /// ```
  pub fn endpoint(&mut self, endpoint: impl Into<String>) {
    self.api_endpoint = endpoint.into();
  }
  /// Endpoint: GET /api/models
  ///
  /// Get information from all models in the Hub
  pub async fn get_models(&self, req: GetModelsReq<'_>) -> GetModelsRes {
    // let req = self.http_client.get()
    todo!()
  }

  /// Endpoint: GET /api/models/{repo_id}
  ///
  /// Endpoint: GET /api/models/{repo_id}/revision/{revision}
  ///
  /// Get all information for a specific model
  pub async fn get_model(&self, req: GetModelReq<'_>) -> GetModelRes {
    todo!()
  }

  /// Endpoint:  POST /api/repos/create
  ///
  /// Create a repository, model repo by default.
  pub async fn create_repo(&self, req: CreateRepoReq<'_>) -> CreateRepoRes {
    todo!()
  }
}
