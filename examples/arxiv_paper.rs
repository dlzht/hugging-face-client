use hugging_face_client::{
  api::ArxivPaperReq,
  client::{Client, ClientOption},
  errors::Result,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
  let access_token = std::env::var("HUGGING_FACE_TOKEN").unwrap();
  let access_proxy = std::env::var("HUGGING_FACE_PROXY").unwrap();
  let option = ClientOption::new(access_token).proxy(access_proxy);
  let client = Client::new(option)?;

  // arxiv paper
  let req = ArxivPaperReq::new("2504.12285");
  let res = client.arxiv_paper(req).await?;
  println!("Arxiv Paper: {:#?}", res);
  Ok(())
}
