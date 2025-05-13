use futures_util::StreamExt;
use hugging_face_client::{
  api::DownloadParquetReq,
  client::{Client, ClientOption},
  errors::Result,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
  let access_token = std::env::var("HUGGING_FACE_TOKEN").unwrap();
  let access_proxy = std::env::var("HUGGING_FACE_PROXY").unwrap();
  let option = ClientOption::new(access_token).proxy(access_proxy);
  let client = Client::new(option)?;

  // download parquet
  let req = DownloadParquetReq::new(
    "DMindAI/DMind_Benchmark",
    "objective_infrastructure",
    "Infrastructrue",
  );
  let mut res = client.download_parquet(req).await;
  // need `use futures_util::StreamExt;`
  while let Some(chunk) = res.next().await {
    println!("{:?}", chunk.map(|b| b.len()).unwrap_or(usize::MAX));
  }

  // download parquet by url
  let url = "https://huggingface.co/api/datasets/DMindAI/DMind_Benchmark/parquet/objective_infrastructure/Infrastructrue/0.parquet";
  let mut res = client.download_parquet_by_url(url).await;
  while let Some(chunk) = res.next().await {
    println!("{:?}", chunk.map(|b| b.len()).unwrap_or(usize::MAX));
  }
  Ok(())
}
