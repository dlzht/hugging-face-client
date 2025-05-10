use reqwest::Method;

use crate::{
  api::{
    CreateRepoReq, CreateRepoRes, DeleteRepoReq, GetDatasetReq, GetDatasetRes, GetDatasetTagRes,
    GetMetricsRes, GetModelReq, GetModelRes, GetModelTagsRes, GetParquetReq, GetParquetRes,
    GetSpaceReq, GetSpaceRes, MoveRepoReq, SearchDatasetReq, SearchDatasetRes, SearchModelReq,
    SearchModelRes, SearchSpaceReq, SearchSpaceRes,
  },
  client::Client,
  errors::Result,
};

impl Client {
  /// Get information from all models in the Hub
  ///
  /// Endpoint: `GET /api/models`
  pub async fn search_model(&self, req: SearchModelReq<'_>) -> Result<SearchModelRes> {
    let url = format!("{}/api/models", &self.api_endpoint);
    self.get_request(&url, Some(&req), true).await
  }

  /// Get all information for a specific model
  ///
  /// Endpoint: `GET /api/models/{repo_id}` or
  ///
  /// Endpoint: `GET /api/models/{repo_id}/revision/{revision}`
  pub async fn get_model(&self, req: GetModelReq<'_>) -> Result<GetModelRes> {
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
  pub async fn get_model_tags(&self) -> Result<GetModelTagsRes> {
    let url = format!("{}/api/models-tags-by-type", &self.api_endpoint);
    let req = if true { None } else { Some(&()) };
    self.get_request(&url, req, false).await
  }

  /// Get information from all datasets in the Hub
  ///
  /// Endpoint: ` GET /api/datasets`
  pub async fn search_dataset(&self, req: SearchDatasetReq<'_>) -> Result<SearchDatasetRes> {
    let url = format!("{}/api/datasets", &self.api_endpoint);
    self.get_request(&url, Some(&req), true).await
  }

  /// Get all information for a specific dataset
  ///
  /// Endpoint: `GET /api/datasets/{repo_id}` or
  ///
  /// Endpoint: `GET /api/datasets/{repo_id}/revision/{revision}`
  pub async fn get_dataset(&self, req: GetDatasetReq<'_>) -> Result<GetDatasetRes> {
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

  /// Gets all the available dataset tags hosted in the Hub
  ///
  /// Endpoint: `GET /api/datasets-tags-by-type`
  pub async fn get_dataset_tags(&self) -> Result<GetDatasetTagRes> {
    let url = format!("{}/api/models-tags-by-type", &self.api_endpoint);
    let req = if true { None } else { Some(&()) };
    self.get_request(&url, req, false).await
  }

  /// Get information from all spaces in the Hub
  ///
  /// Endpoint: ` GET /api/spaces`
  pub async fn search_space(&self, req: SearchSpaceReq<'_>) -> Result<SearchSpaceRes> {
    let url = format!("{}/api/spaces", &self.api_endpoint);
    self.get_request(&url, Some(&req), true).await
  }

  /// Get all information for a specific space
  ///
  /// Endpoint: `GET /api/spaces/{repo_id}` or
  ///
  /// Endpoint: `GET /api/spaces/{repo_id}/revision/{revision}`
  pub async fn get_space(&self, req: GetSpaceReq<'_>) -> Result<GetSpaceRes> {
    let url = if let Some(revision) = req.revision {
      format!(
        "{}/api/spaces/{}/revision/{}",
        &self.api_endpoint, req.repo_name, revision
      )
    } else {
      format!("{}/api/spaces/{}", &self.api_endpoint, req.repo_name)
    };
    let req = if true { None } else { Some(&req) };
    self.get_request(&url, req, true).await
  }

  /// Create a repository, model repo by default
  ///
  /// Endpoint:  POST /api/repos/create
  pub async fn create_repo(&self, req: CreateRepoReq<'_>) -> Result<CreateRepoRes> {
    let url = format!("{}/api/repos/create", &self.api_endpoint);
    self.exec_request(&url, Method::POST, Some(&req)).await
  }

  /// Delete a repository, model repo by default
  ///
  /// Endpoint: `DELETE /api/repos/delete`
  pub async fn delete_repo(&self, req: DeleteRepoReq<'_>) -> Result<()> {
    let url = format!("{}/api/repos/delete", &self.api_endpoint);
    self
      .exec_request_without_response(&url, Method::DELETE, Some(&req))
      .await
  }

  /// Move a repository (rename within the same namespace or transfer from user to organization).
  ///
  /// Endpoint: ` POST /api/repos/move`
  pub async fn move_repo(&self, req: MoveRepoReq<'_>) -> Result<()> {
    let url = format!("{}/api/repos/move", &self.api_endpoint);
    self
      .exec_request_without_response(&url, Method::POST, Some(&req))
      .await
  }

  /// Gets all the available metrics in the Hub
  ///
  /// Endpoint: `GET /api/metrics`
  pub async fn get_metrics(&self) -> Result<GetMetricsRes> {
    let url = format!("{}/api/metrics", &self.api_endpoint);
    let req = if true { None } else { Some(&()) };
    self.get_request(&url, req, false).await
  }

  /// Get the list of auto-converted parquet files
  ///
  /// Endpoint: `GET /api/datasets/{repo_id}/parquet`
  pub async fn get_parquet(&self, req: GetParquetReq<'_>) -> Result<GetParquetRes> {
    let url = format!(
      "{}/api/datasets/{}/parquet",
      &self.api_endpoint, req.repo_name
    );
    let req = if true { None } else { Some(&()) };
    self.get_request(&url, req, false).await
  }
}
