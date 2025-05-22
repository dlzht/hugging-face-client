use serde::{Deserialize, Serialize};

use crate::{repo::RepoType, space::SpaceSdkType};

/// Request of [`crate::client::Client::create_repo`]
#[derive(Debug, Serialize)]
pub struct CreateRepoReq {
  name: String,

  #[serde(rename = "type")]
  repo_type: RepoType,

  #[serde(skip_serializing_if = "Option::is_none")]
  organization: Option<String>,

  private: bool,

  #[serde(skip_serializing_if = "Option::is_none")]
  sdk: Option<SpaceSdkType>,
}

impl CreateRepoReq {
  pub fn new(name: impl Into<String>) -> Self {
    CreateRepoReq {
      name: name.into(),
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

  pub fn organization(mut self, organization: impl Into<String>) -> Self {
    self.organization = Some(organization.into());
    self
  }

  pub fn private(mut self, private: bool) -> Self {
    self.private = private;
    self
  }

  pub fn sdk(mut self, sdk_type: SpaceSdkType) -> Self {
    if matches!(self.repo_type, RepoType::Dataset) {
      self.sdk = Some(sdk_type);
    }
    self
  }
}

impl<T: Into<String>> From<T> for CreateRepoReq {
  fn from(value: T) -> Self {
    Self {
      name: value.into(),
      repo_type: Default::default(),
      organization: None,
      private: false,
      sdk: None,
    }
  }
}
/// Response of [`crate::client::Client::create_repo`]
#[derive(Debug, Deserialize)]
pub struct CreateRepoRes {
  pub id: String,
  pub name: String,
  pub url: String,
}

#[cfg(test)]
mod test {
  use std::assert_matches::assert_matches;

  use crate::{
    api::{CreateRepoReq, CreateRepoRes},
    repo::RepoType,
  };

  #[test]
  fn test_serde_req() {
    let req = CreateRepoReq::new("my-repo")
      .organization("my-org")
      .repo_type(RepoType::Model)
      .private(true);
    let json = serde_json::to_string(&req);
    assert_matches!(json, Ok(v) if v == "{\"name\":\"my-repo\",\"type\":\"model\",\"organization\":\"my-org\",\"private\":true}");
  }

  #[test]
  fn test_serde_res() {
    let json = "{\"url\":\"https://huggingface.co/dlzht/my-repo0\",\"name\":\"dlzht/my-repo0\",\"id\":\"680673d1e332a61dd92e9237\"}";
    let res = serde_json::from_str::<CreateRepoRes>(json);
    assert_matches!(res, Ok(v) if v.id == "680673d1e332a61dd92e9237" && v.name == "dlzht/my-repo0" && v.url == "https://huggingface.co/dlzht/my-repo0");
  }
}
