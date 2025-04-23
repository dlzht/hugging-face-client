use hugging_face_client::{
  api::GetDatasetsReq,
  client::{Client, ClientOption},
  errors::Result,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
  let access_token = std::env::var("HUGGING_FACE_TOKEN").unwrap();
  let access_proxy = std::env::var("HUGGING_FACE_PROXY").unwrap();
  let option = ClientOption::new(access_token).proxy(access_proxy);
  let client = Client::new(option)?;

  // get datasets
  let req = GetDatasetsReq::default().search("awesome-chatgpt-prompts");
  let res = client.search_dataset(req).await?;
  println!("Get Models: {:#?}", res);
  Ok(())
}
