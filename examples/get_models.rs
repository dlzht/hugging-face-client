use hugging_face_client::api::GetModelsReq;
use hugging_face_client::client::{Client, ClientOption};
use hugging_face_client::errors::Result;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
  let access_token = std::env::var("HUGGING_FACE_TOKEN").unwrap();
  let access_proxy = std::env::var("HUGGING_FACE_PROXY").unwrap();
  let option = ClientOption::new(access_token).proxy(access_proxy);
  let client = Client::new(option)?;

  // get models
  let get_models_req = GetModelsReq::default().search("DeepSeek-R1-LOGOSMASTER-HEIDEGGER");
  let res = client.get_models(get_models_req).await?;
  println!("Get Models: {:#?}", res);
  Ok(())
}