use hugging_face_client::{
  api::GetModelReq,
  client::{Client, ClientOption},
  errors::Result,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
  let access_token = std::env::var("HUGGING_FACE_TOKEN").unwrap();
  let access_proxy = std::env::var("HUGGING_FACE_PROXY").unwrap();
  let option = ClientOption::new(access_token).proxy(access_proxy);
  let client = Client::new(option)?;

  // get models
  let get_model_req = GetModelReq::new("microsoft/bitnet-b1.58-2B-4T");
  let res = client.get_model(get_model_req).await?;
  println!("Get Model: {:#?}", res);
  Ok(())
}
