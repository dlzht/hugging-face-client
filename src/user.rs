use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
  pub id: String,

  #[serde(rename = "type")]
  pub user_type: String,

  #[serde(rename = "avatarUrl")]
  pub avatar_url: String,

  #[serde(rename = "fullname")]
  pub full_name: String,

  pub name: String,

  #[serde(rename = "canPay")]
  pub can_pay: bool,

  #[serde(rename = "isPro")]
  pub is_pro: bool,

  pub orgs: Vec<Organization>,
  // #[serde(rename = "isHf")]
  // pub is_hf: bool,
  //
  // #[serde(rename = "isHfAdmin")]
  // pub is_hf_admin: bool,
  //
  // #[serde(rename = "isMod")]
  // pub is_mod: bool,
  //
  // #[serde(rename = "isEnterprise")]
  // pub is_enterprise: Option<bool>,
  //
  // #[serde(rename = "followerCount")]
  // pub follower_count: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organization {
  #[serde(rename = "type")]
  pub org_type: String,

  pub id: String,
  pub name: String,

  #[serde(rename = "avatarUrl")]
  pub avatar_url: String,

  #[serde(rename = "fullname")]
  pub full_name: String,

  #[serde(rename = "isEnterprise")]
  pub is_enterprise: bool,
}
