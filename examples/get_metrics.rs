use hugging_face_client::client::{Client, ClientOption};

#[tokio::main(flavor = "current_thread")]
async fn main() -> hugging_face_client::errors::Result<()> {
  let access_token = std::env::var("HUGGING_FACE_TOKEN").unwrap();
  let access_proxy = std::env::var("HUGGING_FACE_PROXY").unwrap();
  let option = ClientOption::new(access_token).proxy(access_proxy);
  let client = Client::new(option)?;

  // get metrics
  let res = client.get_metrics().await?;
  println!("Get Metrics: {:#?}", res);
  Ok(())
}
