use hugging_face_client::{
  api::DeleteCollectionItemReq,
  client::{Client, ClientOption},
  errors::Result,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
  let access_token = std::env::var("HUGGING_FACE_TOKEN").unwrap();
  let access_proxy = std::env::var("HUGGING_FACE_PROXY").unwrap();
  let option = ClientOption::new(access_token).proxy(access_proxy);
  let client = Client::new(option)?;

  // delete collection item
  let req = DeleteCollectionItemReq::new(
    "dlzht/my-collection01-68199a8c4bf36b6da52840ca",
    "681c21446ce76b62a299f9e8",
  );
  let res = client.delete_collection_item(req).await?;
  println!("Delete Collection Item: {:#?}", res);
  Ok(())
}
