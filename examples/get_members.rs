use hugging_face_client::{
  api::GetMembersReq,
  client::{Client, ClientOption},
  errors::Result,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
  let access_token = std::env::var("HUGGING_FACE_TOKEN").unwrap();
  let access_proxy = std::env::var("HUGGING_FACE_PROXY").unwrap();
  let option = ClientOption::new(access_token).proxy(access_proxy);
  let client = Client::new(option)?;

  // get members
  let req = GetMembersReq::new("dlzht-org");
  let res = client.get_members(req).await?;
  println!("Get Members: {:#?}", res);
  Ok(())
}
