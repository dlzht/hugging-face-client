use serde::{Deserialize, Serialize};
use crate::repo::RepoType;
use crate::space::SpaceSdkType;

/// Request of [`crate::client::Client::create_repo`]
#[derive(Debug, Serialize)]
pub struct CreateRepoReq<'a> {
  name: &'a str,

  #[serde(rename = "type")]
  repo_type: RepoType,

  organization: Option<&'a str>,

  private: bool,

  sdk: Option<SpaceSdkType>,
}

impl<'a> CreateRepoReq<'a> {
  pub fn new(name: &'a str) -> Self {
    CreateRepoReq {
      name,
      repo_type: RepoType::default(),
      organization: None,
      private: false,
      sdk: None,
    }
  }

  pub fn repo_type(mut self, repo_type: RepoType) -> Self {
    self.repo_type = repo_type;
    self
  }

  pub fn organization(mut self, organization: &'a str) -> Self {
    self.organization = Some(organization);
    self
  }

  pub fn private(mut self, private: bool) -> Self {
    self.private = private;
    self
  }

  pub fn sdk(mut self, sdk_type: SpaceSdkType) -> Self {
    self.sdk = Some(sdk_type);
    self
  }
}

/// Response of [`crate::client::Client::create_repo`]
#[derive(Debug, Deserialize)]
pub struct CreateRepoRes {
  id: String,
  name: String,
  url: String,
}

impl CreateRepoRes {
  pub fn id(&self) -> &str {
    &self.id
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn url(&self) -> &str {
    &self.url
  }
}

#[cfg(test)]
mod test {
  use std::assert_matches::assert_matches;
  use crate::api::{CreateRepoReq, CreateRepoRes};
  use crate::repo::RepoType;

  #[test]
  fn test_serde_req() {
    let req = CreateRepoReq::new("my-repo")
      .organization("my-org")
      .repo_type(RepoType::Model)
      .private(true);
    let json = serde_json::to_string(&req);
    assert_matches!(json, Ok(v) if v == "{\"name\":\"my-repo\",\"type\":\"model\",\"organization\":\"my-org\",\"private\":true,\"sdk\":null}");
  }

  #[test]
  fn test_serde_res() {
    let json = "{\"url\":\"https://huggingface.co/dlzht/my-repo0\",\"name\":\"dlzht/my-repo0\",\"id\":\"680673d1e332a61dd92e9237\"}";
    let res = serde_json::from_str::<CreateRepoRes>(json);
    assert_matches!(res, Ok(v) if v.id() == "680673d1e332a61dd92e9237" && v.name == "dlzht/my-repo0" && v.url() == "https://huggingface.co/dlzht/my-repo0");
  }

}