use reqwest::Method;

use crate::{
  api::{
    CreateRepoReq, CreateRepoRes, DeleteRepoReq, GetDatasetReq, GetDatasetRes, GetDatasetsReq,
    GetDatasetsRes, GetModelReq, GetModelRes, GetModelsReq, GetModelsRes, GetTagsRes,
  },
  client::Client,
};

impl Client {
  /// Get information from all models in the Hub
  ///
  /// Endpoint: `GET /api/models`
  pub async fn search_model(&self, req: GetModelsReq<'_>) -> crate::errors::Result<GetModelsRes> {
    let url = format!("{}/api/models", &self.api_endpoint);
    self.get_request(&url, Some(&req), true).await
  }

  /// Get all information for a specific model
  ///
  /// Endpoint: `GET /api/models/{repo_id}` or
  ///
  /// Endpoint: `GET /api/models/{repo_id}/revision/{revision}`
  pub async fn get_model(&self, req: GetModelReq<'_>) -> crate::errors::Result<GetModelRes> {
    let url = if let Some(revision) = req.revision {
      format!(
        "{}/api/models/{}/revision/{}",
        &self.api_endpoint, req.repo_name, revision
      )
    } else {
      format!("{}/api/models/{}", &self.api_endpoint, req.repo_name)
    };
    let req = if true { None } else { Some(&req) };
    self.get_request(&url, req, true).await
  }

  /// Gets all the available model tags hosted in the Hub
  ///
  /// Endpoint: `GET /api/models-tags-by-type`
  pub async fn get_tags(&self) -> crate::errors::Result<GetTagsRes> {
    let url = format!("{}/api/models-tags-by-type", &self.api_endpoint);
    let req = if true { None } else { Some(&()) };
    self.get_request(&url, req, false).await
  }

  /// Get information from all datasets in the Hub
  ///
  /// Endpoint: ` GET /api/datasets`
  pub async fn search_dataset(
    &self,
    req: GetDatasetsReq<'_>,
  ) -> crate::errors::Result<GetDatasetsRes> {
    let url = format!("{}/api/datasets", &self.api_endpoint);
    self.get_request(&url, Some(&req), true).await
  }

  /// Get all information for a specific model
  ///
  /// Endpoint: `GET /api/datasets/{repo_id}` or
  ///
  /// Endpoint: `GET /api/datasets/{repo_id}/revision/{revision}`
  pub async fn get_dataset(&self, req: GetDatasetReq<'_>) -> crate::errors::Result<GetDatasetRes> {
    let url = if let Some(revision) = req.revision {
      format!(
        "{}/api/datasets/{}/revision/{}",
        &self.api_endpoint, req.repo_name, revision
      )
    } else {
      format!("{}/api/datasets/{}", &self.api_endpoint, req.repo_name)
    };
    let req = if true { None } else { Some(&req) };
    self.get_request(&url, req, true).await
  }

  /// Create a repository, model repo by default.
  ///
  /// Endpoint:  POST /api/repos/create
  pub async fn create_repo(&self, req: CreateRepoReq<'_>) -> crate::errors::Result<CreateRepoRes> {
    let url = format!("{}/api/repos/create", &self.api_endpoint);
    self.exec_request(&url, Method::POST, Some(&req)).await
  }

  /// Delete a repository, model repo by default
  ///
  /// Endpoint: `DELETE /api/repos/delete`
  ///
  pub async fn delete_repo(&self, req: DeleteRepoReq<'_>) -> crate::errors::Result<()> {
    let url = format!("{}/api/repos/delete", &self.api_endpoint);
    self
      .exec_request_without_response(&url, Method::DELETE, Some(&req))
      .await
  }
}
