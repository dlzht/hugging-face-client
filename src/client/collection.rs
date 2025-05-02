use crate::{
  api::{GetCollectionReq, GetCollectionRes, GetCollectionsReq, GetCollectionsRes},
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

  /// Get information about a collection
  ///
  /// Endpoint: ` GET /api/collections/{namespace}/{slug}-{id}`
  pub async fn get_collection(&self, req: GetCollectionReq<'_>) -> Result<GetCollectionRes> {
    let url = format!("{}/api/collections/{}", &self.api_endpoint, req.slug);
    let req = if true { None } else { Some(&()) };
    self.get_request(&url, req, false).await
  }
}
