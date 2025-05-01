use crate::{
  api::{GetCollectionsReq, GetCollectionsRes},
  client::Client,
  errors::Result,
};

impl Client {
  /// List collections from the Hub, based on some criteria
  ///
  /// Endpoint: `GET /api/collections`
  pub async fn get_collections(&self, req: GetCollectionsReq<'_>) -> Result<GetCollectionsRes> {
    let url = format!("{}/api/collections", &self.api_endpoint);
    self.get_request(&url, Some(&req), false).await
  }
}
