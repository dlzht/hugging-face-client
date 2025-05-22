use hugging_face_client::{
  RepoType,
  api::CreateRepoReq,
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
  let req = CreateRepoReq::new("example-repo")
    .repo_type(RepoType::Model)
    .private(true);
  let res = client.create_repo(req).await?;
  println!("Create Repo: {:#?}", res);
  
  // for convenient
  let res = client.create_repo("example-repo").await?;
  println!("Create Repo: {:#?}", res);
  Ok(())
}
