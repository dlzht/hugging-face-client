use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sibling {
  #[serde(rename = "rfilename")]
  file_name: String,
}

impl Sibling {
  pub fn new(file_name: impl Into<String>) -> Self {
    Sibling {
      file_name: file_name.into(),
    }
  }

  pub fn file_name(&self) -> &str {
    &self.file_name
  }
}

#[cfg(test)]
mod test {
  use std::assert_matches::assert_matches;

  use crate::sibling::Sibling;

  #[test]
  fn test_serde_sibling() {
    let siblings = Sibling {
      file_name: String::from("file01"),
    };
    let json = serde_json::to_string(&siblings);
    assert_matches!(json, Ok(v) if v == "{\"rfilename\":\"file01\"}");

    let json = "{\"rfilename\":\"file01\"}";
    let sibling = serde_json::from_str::<Sibling>(json);
    assert_matches!(sibling, Ok(v) if v.file_name() == "file01");
  }
}
