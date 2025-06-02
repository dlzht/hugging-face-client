use hugging_face_client::{
  api::GetDatasetReq,
  client::{Client, ClientOption},
  errors::Result,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
  let access_token = std::env::var("HUGGING_FACE_TOKEN").unwrap();
  let access_proxy = std::env::var("HUGGING_FACE_PROXY").unwrap();
  let option = ClientOption::new(access_token).proxy(access_proxy);
  let client = Client::new(option)?;

  // get dataset
  let req = GetDatasetReq::new("fka/awesome-chatgpt-prompts");
  let res = client.get_dataset(req).await?;
  println!("Get Model: {:#?}", res);

  // for convenient
  let res = client.get_dataset("fka/awesome-chatgpt-prompts").await?;
  println!("Get Model: {:#?}", res);
  Ok(())
}
