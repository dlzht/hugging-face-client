use reqwest::Method;

use crate::{
  api::{
    CreateCollectionItemReq, CreateCollectionItemRes, CreateCollectionReq, DeleteCollectionItemReq,
    DeleteCollectionItemRes, DeleteCollectionReq, DeleteCollectionRes, GetCollectionReq,
    GetCollectionRes, GetCollectionsReq, GetCollectionsRes, ModifyCollectionItemReq,
    ModifyCollectionItemRes, ModifyCollectionReq, ModifyCollectionRes,
  },
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
  /// Endpoint: `GET /api/collections/{namespace}/{slug}-{id}`
  pub async fn get_collection(&self, req: GetCollectionReq<'_>) -> Result<GetCollectionRes> {
    let url = format!("{}/api/collections/{}", &self.api_endpoint, req.slug);
    self.get_request(&url, self.empty_req(), false).await
  }

  /// Create a new collection on the Hub with a title
  ///
  /// Endpoint: `POST /api/collections`
  pub async fn create_collection(&self, req: impl Into<CreateCollectionReq>) -> Result<GetCollectionRes> {
    let req = req.into();
    let url = format!("{}/api/collections", &self.api_endpoint);
    self.exec_request(&url, Method::POST, Some(&req)).await
  }

  /// Delete a collection, cannot be restored.
  ///
  /// Endpoint: `DELETE /api/collections/{namespace}/{slug}-{id}`
  pub async fn delete_collection(
    &self,
    req: DeleteCollectionReq<'_>,
  ) -> Result<DeleteCollectionRes> {
    let url = format!("{}/api/collections/{}", &self.api_endpoint, req.slug);
    self.exec_request(&url, Method::DELETE, self.empty_req()).await
  }

  /// Update the metadata of a collection on the Hub
  ///
  /// Endpoint: `PATCH /api/collections/{namespace}/{slug}-{id}`
  pub async fn modify_collection(
    &self,
    req: ModifyCollectionReq<'_>,
  ) -> Result<ModifyCollectionRes> {
    let url = format!("{}/api/collections/{}", &self.api_endpoint, req.slug);
    self.exec_request(&url, Method::PATCH, Some(&req)).await
  }

  /// Add an item to a collection
  ///
  /// Endpoint: `POST /api/collections/{namespace}/{slug}-{id}/items`
  pub async fn create_collection_item(
    &self,
    req: impl Into<CreateCollectionItemReq>,
  ) -> Result<CreateCollectionItemRes> {
    let req = req.into();
    let url = format!(
      "{}/api/collections/{}/items",
      &self.api_endpoint, req.collection_slug
    );
    self.exec_request(&url, Method::POST, Some(&req)).await
  }

  /// Update an item in a collection
  ///
  /// Endpoint: `PATCH /api/collections/{namespace}/{slug}-{id}/items`
  pub async fn modify_collection_item(
    &self,
    req: ModifyCollectionItemReq<'_>,
  ) -> Result<ModifyCollectionItemRes> {
    let url = format!(
      "{}/api/collections/{}/items/{}",
      &self.api_endpoint, req.collection_slug, req.item_id
    );
    self.exec_request(&url, Method::PATCH, Some(&req)).await
  }

  /// Remove an item from a collection
  ///
  /// Endpoint: `Delete /api/collections/{namespace}/{slug}-{id}/items`
  pub async fn delete_collection_item(
    &self,
    req: DeleteCollectionItemReq<'_>,
  ) -> Result<DeleteCollectionItemRes> {
    let url = format!(
      "{}/api/collections/{}/items/{}",
      &self.api_endpoint, req.collection_slug, req.item_id
    );
    self.exec_request(&url, Method::DELETE, self.empty_req()).await
  }
}
