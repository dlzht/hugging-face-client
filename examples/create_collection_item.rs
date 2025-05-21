use hugging_face_client::{
  api::CreateCollectionItemReq,
  client::{Client, ClientOption},
  errors::Result,
  RepoType,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
  let access_token = std::env::var("HUGGING_FACE_TOKEN").unwrap();
  let access_proxy = std::env::var("HUGGING_FACE_PROXY").unwrap();
  let option = ClientOption::new(access_token).proxy(access_proxy);
  let client = Client::new(option)?;

  // create collection item
  let req = CreateCollectionItemReq::new(
    "dlzht/my-collection01-68199a8c4bf36b6da52840ca",
    "microsoft/bitnet-b1.58-2B-4T",
    RepoType::Model,
  )
  .note("Add microsoft/bitnet-b1.58-2B-4T to my collection");
  let res = client.create_collection_item(req).await?;
  println!("Create Collection Item: {:#?}", res);

  // for convenient
  let res = client
    .create_collection_item((
      "dlzht/my-collection01-68199a8c4bf36b6da52840ca",
      "microsoft/bitnet-b1.58-2B-4T",
      RepoType::Model,
    ))
    .await?;
  println!("Create Collection Item: {:#?}", res);

  Ok(())
}
