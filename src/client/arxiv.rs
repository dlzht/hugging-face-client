use crate::{
  api::{ArxivPaperReq, ArxivPaperRes, ArxivReposReq, ArxivReposRes},
  client::Client,
  errors::Result,
};

impl Client {
  /// Get the API equivalent of the Paper page, i.e., metadata like authors, summary, and discussion
  /// comments
  ///
  /// Endpoint: ` GET /api/papers/{arxiv_id}`
  pub async fn arxiv_paper(&self, req: ArxivPaperReq<'_>) -> Result<ArxivPaperRes> {
    let url = format!("{}/api/papers/{}", &self.api_endpoint, req.paper_id);
    let req = if true { None } else { Some(&()) };
    self.get_request(&url, req, false).await
  }

  /// Get all the models, datasets, and Spaces that refer to a paper
  ///
  /// Endpoint: ` GET /api/arxiv/{arxiv_id}/repos`
  pub async fn arxiv_repos(&self, req: ArxivReposReq<'_>) -> Result<ArxivReposRes> {
    let url = format!("{}/api/arxiv/{}/repos", &self.api_endpoint, req.paper_id);
    let req = if true { None } else { Some(&()) };
    self.get_request(&url, req, true).await
  }
}
