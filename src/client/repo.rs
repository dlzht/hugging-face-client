use bytes::Bytes;
use futures_core::Stream;
use reqwest::Method;

use crate::{
  api::{
    CreateRepoReq, CreateRepoRes, DeleteRepoReq, DownloadParquetReq, DownloadParquetRes,
    GetDatasetReq, GetDatasetRes, GetDatasetTagRes, GetMetricsRes, GetModelReq, GetModelRes,
    GetModelTagsRes, GetParquetReq, GetParquetRes, GetSpaceReq, GetSpaceRes, MoveRepoReq,
    SearchDatasetReq, SearchDatasetRes, SearchModelReq, SearchModelRes, SearchSpaceReq,
    SearchSpaceRes,
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
    self.get_request(&url, self.empty_req(), true).await
  }

  /// Gets all the available model tags hosted in the Hub
  ///
  /// Endpoint: `GET /api/models-tags-by-type`
  pub async fn get_model_tags(&self) -> Result<GetModelTagsRes> {
    let url = format!("{}/api/models-tags-by-type", &self.api_endpoint);
    self.get_request(&url, self.empty_req(), false).await
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
    self.get_request(&url, self.empty_req(), true).await
  }

  /// Gets all the available dataset tags hosted in the Hub
  ///
  /// Endpoint: `GET /api/datasets-tags-by-type`
  pub async fn get_dataset_tags(&self) -> Result<GetDatasetTagRes> {
    let url = format!("{}/api/models-tags-by-type", &self.api_endpoint);
    self.get_request(&url, self.empty_req(), false).await
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
    self.get_request(&url, self.empty_req(), true).await
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
    self.get_request(&url, self.empty_req(), false).await
  }

  /// Get the list of auto-converted parquet files
  ///
  /// Endpoint: `GET /api/datasets/{repo_id}/parquet`
  ///
  /// Endpoint: `GET /api/datasets/{repo_id}/parquet/{subset}`
  ///
  /// Endpoint: `GET /api/datasets/{repo_id}/parquet/{subset}/{split}`
  pub async fn get_parquet(&self, req: GetParquetReq<'_>) -> Result<GetParquetRes> {
    let url = match (req.subset, req.split) {
      (None, None) | (None, Some(_)) => format!(
        "{}/api/datasets/{}/parquet",
        &self.api_endpoint, req.repo_name
      ),
      (Some(subset), None) => format!(
        "{}/api/datasets/{}/parquet/{}",
        &self.api_endpoint, req.repo_name, subset
      ),
      (Some(subset), Some(split)) => format!(
        "{}/api/datasets/{}/parquet/{}/{}",
        &self.api_endpoint, req.repo_name, subset, split
      ),
    };
    self.get_request(&url, self.empty_req(), false).await
  }

  /// Get the nth shard of the auto-converted parquet files, for a specific subset (also called
  /// “config”) and split.
  ///
  /// Endpoint: ` GET /api/datasets/{repo_id}/parquet/{subset}/{split}/{n}.parquet`
  pub async fn download_parquet(
    &self,
    req: DownloadParquetReq<'_>,
  ) -> impl Stream<Item = Result<Bytes>> {
    let url = format!(
      "{}/api/datasets/{}/parquet/{}/{}/{}.parquet",
      &self.api_endpoint, req.repo_name, req.subset, req.split, req.nth
    );
    let stream = self
      .http_client
      .get(url)
      .bearer_auth(&self.access_token)
      .send()
      .await
      .unwrap()
      .bytes_stream();
    DownloadParquetRes::new(stream)
  }

  ///  Get the nth shard of the auto-converted parquet files, same as [`Client::download_parquet`]
  ///
  /// `url`: full url of parquet file, you can get this from [`Client::get_parquet`],
  /// this is an example: `https://huggingface.co/api/datasets/DMindAI/DMind_Benchmark/parquet/objective_infrastructure/Infrastructrue/0.parquet`
  pub async fn download_parquet_by_url(
    &self,
    url: impl AsRef<str>,
  ) -> impl Stream<Item = Result<Bytes>> {
    let stream = self
      .http_client
      .get(url.as_ref())
      .bearer_auth(&self.access_token)
      .send()
      .await
      .unwrap()
      .bytes_stream();
    DownloadParquetRes::new(stream)
  }
}
