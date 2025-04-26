use crate::{
  api::{ArxivPaperReq, ArxivPaperRes},
  client::Client,
  errors::Result,
};

impl Client {

  /// Get the API equivalent of the Paper page, i.e., metadata like authors, summary, and discussion
  /// comments.
  ///
  /// Endpoint: ` GET /api/papers/{arxiv_id}`
  pub async fn arxiv_paper(&self, req: ArxivPaperReq<'_>) -> Result<ArxivPaperRes> {
    let url = format!("{}/api/papers/{}", &self.api_endpoint, req.paper_id);
    let req = if true { None } else { Some(&()) };
    self.get_request(&url, req, false).await
  }
}
