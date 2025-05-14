use crate::{
  api::{GetMembersReq, GetMembersRes},
  client::Client,
  errors::Result,
};

impl Client {
  /// Get a list of the Organization members.
  ///
  /// Endpoint: ` GET /api/organizations/{organization_name}/members`
  pub async fn get_members(&self, req: GetMembersReq<'_>) -> Result<GetMembersRes> {
    let url = format!(
      "{}/api/organizations/{}/members",
      &self.api_endpoint, req.org_name
    );
    self.get_request(&url, self.empty_req(), true).await
  }
}
