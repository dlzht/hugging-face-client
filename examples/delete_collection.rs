use hugging_face_client::{
  api::DeleteCollectionReq,
  client::{Client, ClientOption},
  errors::Result,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
  let access_token = std::env::var("HUGGING_FACE_TOKEN").unwrap();
  let access_proxy = std::env::var("HUGGING_FACE_PROXY").unwrap();
  let option = ClientOption::new(access_token).proxy(access_proxy);
  let client = Client::new(option)?;

  // delete collection
  let req = DeleteCollectionReq::new("dlzht/my-collection-68174f008e267618d9d3d474");
  let res = client.delete_collection(req).await?;
  println!("Delete Collection: {:#?}", res);

  // for convenient
  let res = client
    .delete_collection("dlzht/my-collection-68174f008e267618d9d3d474")
    .await?;
  println!("Delete Collection: {:#?}", res);
  Ok(())
}
