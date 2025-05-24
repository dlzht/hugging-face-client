use hugging_face_client::{
  api::CreateCollectionReq,
  client::{Client, ClientOption},
  errors::Result,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
  let access_token = std::env::var("HUGGING_FACE_TOKEN").unwrap();
  let access_proxy = std::env::var("HUGGING_FACE_PROXY").unwrap();
  let option = ClientOption::new(access_token).proxy(access_proxy);
  let client = Client::new(option)?;

  // create collection
  let req = CreateCollectionReq::new("my-collection-item05", "dlzht", true)
    .description("Item05 of My Collection");
  let res = client.create_collection(req).await?;
  println!("Create Collection: {:#?}", res);

  // for convenient
  let res = client
    .create_collection(("my-collection-item05", "dlzht", true))
    .await?;
  println!("Create Collection: {:#?}", res);

  Ok(())
}
