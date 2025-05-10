use hugging_face_client::{
  api::{GetDatasetReq, GetParquetReq},
  client::{Client, ClientOption},
  errors::Result,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
  let access_token = std::env::var("HUGGING_FACE_TOKEN").unwrap();
  let access_proxy = std::env::var("HUGGING_FACE_PROXY").unwrap();
  let option = ClientOption::new(access_token).proxy(access_proxy);
  let client = Client::new(option)?;

  // get parquet
  let req = GetParquetReq::new("fka/awesome-chatgpt-prompts");
  let res = client.get_parquet(req).await?;
  println!("Get Parquet: {:#?}", res);
  Ok(())
}
