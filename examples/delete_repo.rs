use hugging_face_client::{
  RepoType,
  api::DeleteRepoReq,
  client::{Client, ClientOption},
  errors::Result,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
  let access_token = std::env::var("HUGGING_FACE_TOKEN").unwrap();
  let access_proxy = std::env::var("HUGGING_FACE_PROXY").unwrap();
  let option = ClientOption::new(access_token).proxy(access_proxy);
  let client = Client::new(option)?;

  // create repo
  let req = DeleteRepoReq::new("example-repo").repo_type(RepoType::Model);
  let _ = client.delete_repo(req).await;

  // for convenient
  client.delete_repo("example-repo").await
}
