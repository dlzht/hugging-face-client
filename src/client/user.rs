use crate::{api::GetUserInfoRes, client::Client, errors::Result};

impl Client {
  /// Get username and organizations the user belongs to
  ///
  /// Endpoint: `GET /api/whoami-v2`
  pub async fn get_userinfo(&self) -> Result<GetUserInfoRes> {
    let url = format!("{}/api/whoami-v2", &self.api_endpoint);
    let req = if true { None } else { Some(&()) };
    self.get_request(&url, req, true).await
  }
}
