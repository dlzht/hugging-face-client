use crate::api::{CreateRepoReq, CreateRepoRes, GetModelReq, GetModelRes, GetModelsReq, GetModelsRes};

/// Async client for Hugging Face Hub API
pub struct Client {

}

impl Client {

  /// Endpoint: GET /api/models
  ///
  /// Get information from all models in the Hub
  pub async fn get_models(&self, req: GetModelsReq<'_>) -> GetModelsRes {
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