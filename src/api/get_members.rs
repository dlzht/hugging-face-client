use serde::Deserialize;

/// Request of [`crate::client::Client::get_members`]
pub struct GetMembersReq<'a> {
  pub(crate) org_name: &'a str,
}

impl<'a> GetMembersReq<'a> {
  pub fn new(org_name: &'a str) -> Self {
    GetMembersReq { org_name }
  }
}

/// Response of [`crate::client::Client::get_members`]
pub type GetMembersRes = Vec<GetMembersItem>;

#[derive(Debug, Deserialize)]
pub struct GetMembersItem {
  #[serde(rename = "_id")]
  pub id: String,

  #[serde(rename = "avatarUrl")]
  pub avatar_url: String,

  #[serde(rename = "fullname")]
  pub full_name: String,

  #[serde(rename = "isPro")]
  pub is_pro: bool,

  pub user: String,

  #[serde(rename = "type")]
  pub user_type: String,

  #[serde(rename = "isFollowing")]
  pub is_following: bool,
}
