use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
  #[serde(rename = "avatarUrl")]
  pub avatar_url: String,

  #[serde(rename = "fullname")]
  pub full_name: String,

  pub name: String,

  #[serde(rename = "type")]
  pub author_type: String,

  #[serde(rename = "isHf")]
  pub is_hf: bool,

  #[serde(rename = "isHfAdmin")]
  pub is_hf_admin: bool,

  #[serde(rename = "isMod")]
  pub is_mod: bool,

  #[serde(rename = "isEnterprise")]
  pub is_enterprise: Option<bool>,

  #[serde(rename = "followerCount")]
  pub follower_count: Option<usize>,
}
