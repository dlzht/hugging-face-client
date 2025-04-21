use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelTag {
  inner: TagInner,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum TagInner {
  Defined,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum DefinedTag {}
