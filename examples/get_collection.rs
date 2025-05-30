use hugging_face_client::{
  api::GetCollectionReq,
  client::{Client, ClientOption},
  errors::Result,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
  let access_token = std::env::var("HUGGING_FACE_TOKEN").unwrap();
  let access_proxy = std::env::var("HUGGING_FACE_PROXY").unwrap();
  let option = ClientOption::new(access_token).proxy(access_proxy);
  let client = Client::new(option)?;

  // get collection
  let req = GetCollectionReq::new("facebook/perception-lm-67f9783f171948c383ee7498");
  let res = client.get_collection(req).await?;
  println!("Get Collection: {:#?}", res);

  // for convenient
  let res = client
    .get_collection("facebook/perception-lm-67f9783f171948c383ee7498")
    .await?;
  println!("Get Collection: {:#?}", res);
  Ok(())
}
