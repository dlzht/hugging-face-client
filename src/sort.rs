use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SortField {
  #[serde(rename = "downloads")]
  Downloads,

  #[serde(rename = "likes")]
  Likes,

  #[serde(rename = "trendingScore")]
  TrendingScore,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
pub enum SortDirection {
  Descending = -1,
  // Ascending,
}

#[cfg(test)]
mod test {
  use std::assert_matches::assert_matches;

  use crate::sort::{SortDirection, SortField};

  #[test]
  fn test_serde_sort_direction() {
    let direction = SortDirection::Descending;
    let json = serde_json::to_string(&direction);
    assert_matches!(json, Ok(v) if v == "-1");

    let json = "-1";
    let direction = serde_json::from_str::<SortDirection>(json);
    assert_matches!(direction, Ok(SortDirection::Descending))
  }

  #[test]
  fn test_serde_sort_field() {
    let field = SortField::Downloads;
    let json = serde_json::to_string(&field);
    assert_matches!(json, Ok(v) if v == "\"downloads\"");

    let json = "\"downloads\"";
    let field = serde_json::from_str::<SortField>(json);
    assert_matches!(field, Ok(SortField::Downloads));

    let field = SortField::Likes;
    let json = serde_json::to_string(&field);
    assert_matches!(json, Ok(v) if v == "\"likes\"");

    let json = "\"likes\"";
    let field = serde_json::from_str::<SortField>(json);
    assert_matches!(field, Ok(SortField::Likes));

    let field = SortField::TrendingScore;
    let json = serde_json::to_string(&field);
    assert_matches!(json, Ok(v) if v == "\"trendingScore\"");

    let json = "\"trendingScore\"";
    let field = serde_json::from_str::<SortField>(json);
    assert_matches!(field, Ok(SortField::TrendingScore));
  }
}
