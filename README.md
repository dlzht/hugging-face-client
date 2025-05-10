[![Crates.io](https://img.shields.io/crates/v/hugging-face-client)](https://crates.io/crates/hugging-face-client)
[![Docs.rs](https://img.shields.io/docsrs/hugging-face-client)](https://docs.rs/hugging-face-client)
[![License](https://img.shields.io/github/license/dlzht/hugging-face-client.svg)](https://img.shields.io/github/license/用户名/仓库名.svg)

## Cargo.toml

```toml
hugging-face-client = "0.5"
```

## Example

### Create Client

```rust
use std::time::Duration;
use hugging_face_client::client::{Client, ClientOption};

#[tokio::main(flavor = "current_thread")]
async fn main() {
  let access_token = std::env::var("HUGGING_FACE_TOKEN").unwrap();
  let access_proxy = std::env::var("HUGGING_FACE_PROXY").unwrap();
  let option = ClientOption::new(access_token)
    .proxy(access_proxy)
    .timeout(Duration::from_secs(5));
  let client = Client::new(option).unwrap();
}
```

### Get Model

```rust
use hugging_face_client::api::GetModelReq;
use hugging_face_client::client::{Client, ClientOption};

#[tokio::main(flavor = "current_thread")]
async fn main() {
  let option = ClientOption::new("HUGGING_FACE_TOKEN");
  let client = Client::new(option).unwrap();

  // get model
  let req = GetModelReq::new("microsoft/bitnet-b1.58-2B-4T");
  let res = client.get_model(req).await.unwrap();
  println!("{:#?}", res);
  Ok(())
}
// Model { _id: "67fddfa9a7fe1f21ec1d3026", id: "microsoft/bitnet-b1.58-2B-4T", model_id: None ... }
```

**More examples can be seen [hugging-face-client/examples](https://github.com/dlzht/hugging-face-client/tree/main/examples)**

## API

### Repo listing
+ ✅ GET /api/models 
+ ✅ GET /api/models/{repo_id} or /api/models/{repo_id}/revision/{revision}
+ ✅ GET /api/models-tags-by-type
+ ✅ GET /api/datasets
+ ✅ GET /api/datasets/{repo_id} or /api/datasets/{repo_id}/revision/{revision}
+ ✅ GET /api/datasets/{repo_id}/parquet
+ ❌ GET /api/datasets/{repo_id}/parquet/{subset}/{split}/{n}.parquet
+ ❌ GET /api/datasets/{repo_id}/croissant
+ ✅ GET /api/datasets-tags-by-type
+ ✅ GET /api/spaces
+ ✅ GET /api/spaces/{repo_id} or /api/spaces/{repo_id}/revision/{revision} 
+ ✅ GET /api/metrics

### Repo
+ ✅ POST /api/repos/create
+ ✅ DELETE /api/repos/delete
+ ❔ PUT /api/repos/{repo_type}/{repo_id}/settings
+ ✅ POST /api/repos/move

### User API
+ ✅ GET /api/whoami-v2

### Organization API
+ ✅ GET /api/organizations/{organization_name}/members

### Resource Groups
+ ❌ GET /api/organizations/{name}/resource-groups
+ ❌ GET /api/organizations/{name}/resource-groups/{resourceGroupId}
+ ❌ POST /api/organizations/{name}/resource-groups
+ ❌ PATCH /api/organizations/{name}/resource-groups/{resourceGroupId}
+ ❌ POST /api/organizations/{name}/resource-groups/{resourceGroupId}/settings
+ ❌ DELETE /api/organizations/{name}/resource-groups/{resourceGroupId}
+ ❌ POST /api/organizations/{name}/resource-groups/{resourceGroupId}/users
+ ❌ DELETE /api/organizations/{name}/resource-groups/{resourceGroupId}/users/{username}
+ ❌ PATCH /api/organizations/{name}/resource-groups/{resourceGroupId}/users/{username}
+ ❌ POST /api/(models|spaces|datasets)/{namespace}/{repo}/resource-group
+ ❌ GET /api/(models|spaces|datasets)/{namespace}/{repo}/resource-group

### Paper Pages
+ ✅ GET /api/papers/{arxiv_id}
+ ✅ GET /api/arxiv/{arxiv_id}/repos
+ ✅ GET /api/daily_papers

### Collections
+ ✅ POST /api/collections
+ ✅ GET /api/collections/{namespace}/{slug}-{id}
+ ✅ GET /api/collections
+ ✅ PATCH /api/collections/{namespace}/{slug}-{id}
+ ✅ DELETE /api/collections/{namespace}/{slug}-{id}
+ ✅ POST /api/collections/{namespace}/{slug}-{id}/item
+ ✅ PATCH /api/collections/{namespace}/{slug}-{id}/items/{item_id}
+ ✅ DELETE /api/collections/{namespace}/{slug}-{id}/items/{item_id}