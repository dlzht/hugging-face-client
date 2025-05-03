use serde::{Deserialize, Serialize};

use crate::{
  RepoType, provider::InferenceProvider, runtime::Runtime, space::SpaceSdkType, user::User,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
  pub slug: String,
  pub title: String,
  pub description: Option<String>,
  pub gating: bool,

  #[serde(rename = "lastUpdated")]
  pub last_updated: String,

  pub owner: User,
  pub items: Vec<CollectionItem>,
  pub theme: String,
  pub private: bool,
  pub upvote: Option<usize>,
  // #[serde(rename = "isUpvotedByUser")]
  // is_upvoted_by_user: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionItem {
  #[serde(rename = "_id")]
  pub id: String,

  #[serde(rename = "id")]
  pub name: String,

  pub position: usize,

  #[serde(rename = "type")]
  pub item_type: RepoType,

  // pub repo_type: RepoType,
  pub author: String,

  #[serde(rename = "authorData")]
  pub author_data: Option<User>,

  #[serde(rename = "createAt")]
  pub create_time: Option<String>,

  #[serde(rename = "lastModified")]
  pub last_modified: String,

  pub likes: usize,

  #[serde(rename = "trendingScore")]
  pub trending_score: Option<usize>,

  pub private: bool,
  pub sdk: Option<SpaceSdkType>,
  pub runtime: Option<Runtime>,
  pub title: Option<String>,

  // pub is_liked_by_user: bool
  // pub ai_category: Option<String>,
  // pub ai_short_description: Option<String?,
  pub available_inference_providers: Option<Vec<InferenceProvider>>,
}
