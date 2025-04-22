## Async Implement of Hugging Face Hub API

### Example

```rust
use hugging_face_client::api::GetModelReq;
use hugging_face_client::client::{Client, ClientOption};
use hugging_face_client::errors::Result;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
  let access_token = std::env::var("HUGGING_FACE_TOKEN").unwrap();
  let access_proxy = std::env::var("HUGGING_FACE_PROXY").unwrap();
  let option = ClientOption::new(access_token).proxy(access_proxy);
  let client = Client::new(option)?;

  // get models
  let get_model_req = GetModelReq::new("microsoft/bitnet-b1.58-2B-4T");
  let res = client.get_model(get_model_req).await?;
  println!("{:#?}", res);
  Ok(())
}

// Model { _id: "67fddfa9a7fe1f21ec1d3026", id: "microsoft/bitnet-b1.58-2B-4T", model_id: None ... }
```

**More usage examples, can be seen [hugging-face-client/examples](https://github.com/dlzht/hugging-face-client/tree/main/examples)**