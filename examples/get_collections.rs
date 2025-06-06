use hugging_face_client::{
  api::{GetCollectionsReq, GetMembersReq},
  client::{Client, ClientOption},
  errors::Result,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
  let access_token = std::env::var("HUGGING_FACE_TOKEN").unwrap();
  let access_proxy = std::env::var("HUGGING_FACE_PROXY").unwrap();
  let option = ClientOption::new(access_token).proxy(access_proxy);
  let client = Client::new(option)?;

  // get collections
  let req = GetCollectionsReq::new().owner("facebook").limit(2);
  let res = client.get_collections(req).await?;
  println!("Get Collections: {:#?}", res);
  Ok(())
}
