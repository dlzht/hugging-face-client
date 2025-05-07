use hugging_face_client::{
  RepoType,
  api::{CreateCollectionItemReq, ModifyCollectionItemReq},
  client::{Client, ClientOption},
  errors::Result,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
  let access_token = std::env::var("HUGGING_FACE_TOKEN").unwrap();
  let access_proxy = std::env::var("HUGGING_FACE_PROXY").unwrap();
  let option = ClientOption::new(access_token).proxy(access_proxy);
  let client = Client::new(option)?;

  // modify collection item
  let req = ModifyCollectionItemReq::new(
    "dlzht/my-collection01-68199a8c4bf36b6da52840ca",
    "681ad073b02f43b4b5bad154",
  )
  .note("Modified note");
  let res = client.modify_collection_item(req).await?;
  println!("Modify Collection Item: {:#?}", res);
  Ok(())
}
