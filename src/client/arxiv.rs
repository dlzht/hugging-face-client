use crate::{
  api::{ArxivDailyRes, ArxivPaperReq, ArxivPaperRes, ArxivReposReq, ArxivReposRes},
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
    self.get_request(&url, self.empty_req(), false).await
  }

  /// Get all the models, datasets, and Spaces that refer to a paper
  ///
  /// Endpoint: ` GET /api/arxiv/{arxiv_id}/repos`
  pub async fn arxiv_repos(&self, req: ArxivReposReq<'_>) -> Result<ArxivReposRes> {
    let url = format!("{}/api/arxiv/{}/repos", &self.api_endpoint, req.paper_id);
    self.get_request(&url, self.empty_req(), true).await
  }

  /// Get the daily papers curated by AK and the community
  ///
  /// Endpoint: `GET /api/daily_papers`
  ///
  /// TODO: support filter on date
  pub async fn arxiv_daily(&self) -> Result<ArxivDailyRes> {
    let url = format!("{}/api/daily_papers", &self.api_endpoint);
    self.get_request(&url, self.empty_req(), false).await
  }
}
