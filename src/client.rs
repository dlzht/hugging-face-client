//! Async hub client

use std::time::Duration;

use reqwest::{Client as ReqwestClient, Proxy as ReqwestProxy};
use serde::{Deserialize, Serialize};
use snafu::ResultExt;

use crate::{
  api::{CreateRepoReq, CreateRepoRes, GetModelReq, GetModelRes, GetModelsReq, GetModelsRes},
  errors::{ReqwestClientSnafu, Result},
};
use crate::api::HuggingFaceRes;

const DEFAULT_API_ENDPOINT: &'static str = "https://huggingface.co";

/// Options for creating [`Client`]
#[derive(Debug, Clone)]
pub struct ClientOption {
  access_token: String,
  api_endpoint: Option<String>,
  http_proxy: Option<String>,
  timeout: Option<Duration>,
}

impl ClientOption {
  /// Create [`ClientOption`] instance with access_token
  ///
  /// `access_token`: authenticate client to the Hugging Face Hub and allow client to perform
  /// actions based on token permissions, see [https://huggingface.co/settings/tokens](https://huggingface.co/settings/tokens)
  ///
  /// ```rust
  /// use hugging_face_client::client::ClientOption;
  ///
  /// fn main() {
  ///   let option = ClientOption::new("HUGGING_FACE_TOKEN");
  /// }
  /// ```
  pub fn new(access_token: impl Into<String>) -> Self {
    ClientOption {
      access_token: access_token.into(),
      api_endpoint: None,
      http_proxy: None,
      timeout: None,
    }
  }

  /// Set endpoint, default is `https://huggingface.co`
  ///
  /// ```rust
  /// use hugging_face_client::client::ClientOption;
  ///
  /// fn main() {
  ///   let option = ClientOption::new("HUGGING_FACE_TOKEN")
  ///     .endpoint("https://fast-proxy.huggingface.to");
  /// }
  /// ```
  pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
    let endpoint = endpoint.into().trim().trim_end_matches('/').to_string();
    if endpoint.starts_with("http://") || endpoint.starts_with("https://") {
      self.api_endpoint = Some(endpoint.into());
      return self;
    }
    let mut result = String::with_capacity(endpoint.len() + 8);
    result.push_str("https://");
    result.push_str(endpoint.as_str());
    self.api_endpoint = Some(result);
    self
  }

  /// Set proxy, default is `None`
  ///
  /// ```rust
  /// use hugging_face_client::client::ClientOption;
  ///
  /// fn main() {
  ///   let option = ClientOption::new("HUGGING_FACE_TOKEN")
  ///     .proxy("socks5://127.0.0.1:9000");
  /// }
  /// ```
  pub fn proxy(mut self, proxy: impl Into<String>) -> Self {
    self.http_proxy = Some(proxy.into());
    self
  }

  /// Set timeout in second, default is `None`
  ///
  /// ```rust
  /// use hugging_face_client::client::ClientOption;
  /// use std::time::Duration;
  ///
  /// fn main() {
  /// let option = ClientOption::new("HUGGING_FACE_TOKEN")
  ///     .timeout(Duration::from_secs(5));
  /// }
  /// ```
  pub fn timeout(mut self, timeout: Duration) -> Self {
    self.timeout = Some(timeout);
    self
  }
}

/// Async client for Hugging Face Hub
#[derive(Debug, Clone)]
pub struct Client {
  access_token: String,
  api_endpoint: String,
  http_client: ReqwestClient,
}

impl Client {
  /// Create [`Client`] instance with [`ClientOption`]
  ///
  ///
  /// ```rust
  /// use hugging_face_client::client::Client;
  /// use hugging_face_client::client::ClientOption;
  /// use std::time::Duration;
  ///
  /// fn main() {
  ///   let option = ClientOption::new("HUGGING_FACE_TOKEN")
  ///     .timeout(Duration::from_secs(5));
  ///   let client = Client::new(option);
  /// }
  /// ```
  pub fn new(option: ClientOption) -> Result<Self> {
    let mut http_client = ReqwestClient::builder();
    if let Some(http_proxy) = option.http_proxy {
      let proxy = ReqwestProxy::all(&http_proxy).context(ReqwestClientSnafu)?;
      http_client = http_client.proxy(proxy);
    }
    if let Some(timeout) = option.timeout {
      http_client = http_client.timeout(timeout);
    }
    let http_client = http_client.build().context(ReqwestClientSnafu)?;
    let client = Client {
      access_token: option.access_token,
      api_endpoint: option
        .api_endpoint
        .unwrap_or_else(|| DEFAULT_API_ENDPOINT.to_string()),
      http_client,
    };
    Ok(client)
  }

  /// Endpoint: GET /api/models
  ///
  /// Get information from all models in the Hub
  pub async fn get_models(&self, req: GetModelsReq<'_>) -> Result<GetModelsRes> {
    let url = format!("{}/api/models", &self.api_endpoint);
    self.get_request(&url, Some(&req)).await
  }

  /// Endpoint: GET /api/models/{repo_id}
  ///
  /// Endpoint: GET /api/models/{repo_id}/revision/{revision}
  ///
  /// Get all information for a specific model
  pub async fn get_model(&self, req: GetModelReq<'_>) -> Result<GetModelRes> {
    let url = if let Some(revision) = req.revision {
      format!(
        "{}/api/models/{}/revision/{}",
        &self.api_endpoint, req.repo_id, revision
      )
    } else {
      format!("{}/api/models/{}", &self.api_endpoint, req.repo_id)
    };
    let req = if true { None } else { Some(&req) };
    self.get_request(&url, req).await
  }

  /// Endpoint:  POST /api/repos/create
  ///
  /// Create a repository, model repo by default.
  pub async fn create_repo(&self, req: CreateRepoReq<'_>) -> Result<CreateRepoRes> {
    let url = format!("{}/api/repos/create", &self.api_endpoint);
    self.post_request(&url, Some(&req)).await
  }
}

// private method
impl Client {
  async fn get_request<T: Serialize, U: for<'de> Deserialize<'de>>(
    &self,
    url: &str,
    query: Option<&T>,
  ) -> Result<U> {
    let mut req = self.http_client.get(url).bearer_auth(&self.access_token);
    if let Some(query) = query {
      req = req.query(query);
    }
    let res = req
      .send()
      .await
      .context(ReqwestClientSnafu)?
      .json::<HuggingFaceRes<U>>()
      .await
      .context(ReqwestClientSnafu)?
      .unwrap_data()?;
    Ok(res)
  }

  async fn post_request<T: Serialize, U: for<'de> Deserialize<'de>>(
    &self,
    url: &str,
    body: Option<&T>,
  ) -> Result<U> {
    let mut req = self.http_client.post(url).bearer_auth(&self.access_token);
    if let Some(body) = body {
      req = req.json(body);
    }
    let res = req
      .send()
      .await
      .context(ReqwestClientSnafu)?
      .json::<HuggingFaceRes<U>>()
      .await
      .context(ReqwestClientSnafu)?
      .unwrap_data()?;
    Ok(res)
  }
}
