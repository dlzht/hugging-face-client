use std::{
  pin::Pin,
  task::{Context, Poll},
};

use bytes::Bytes;
use futures_core::Stream;
use pin_project_lite::pin_project;
use serde::Serialize;
use snafu::ResultExt;

use crate::errors::{ReqwestClientSnafu, Result};

/// Request of [`crate::client::Client::download_parquet`]
#[derive(Debug, Serialize)]
pub struct DownloadParquetReq {
  #[serde(rename = "repo_id")]
  pub(crate) repo_name: String,

  pub(crate) subset: String,

  pub(crate) split: String,

  pub(crate) nth: usize,
}

impl DownloadParquetReq {
  pub fn new(
    repo_name: impl Into<String>,
    subset: impl Into<String>,
    split: impl Into<String>,
  ) -> Self {
    DownloadParquetReq {
      repo_name: repo_name.into(),
      subset: subset.into(),
      split: split.into(),
      nth: 0,
    }
  }

  pub fn nth(mut self, nth: usize) -> Self {
    self.nth = nth;
    self
  }
}

impl<T: Into<String>, U: Into<String>, P: Into<String>> From<(T, U, P)> for DownloadParquetReq {
  fn from(value: (T, U, P)) -> Self {
    Self {
      repo_name: value.0.into(),
      subset: value.1.into(),
      split: value.2.into(),
      nth: 0,
    }
  }
}

pin_project! {
  pub struct DownloadParquetRes<T> {
    #[pin]
    stream: T,
  }
}

impl<T> DownloadParquetRes<T> {
  pub(crate) fn new(stream: T) -> DownloadParquetRes<T> {
    DownloadParquetRes { stream }
  }

  pub fn get_stream(self) -> T {
    self.stream
  }
}

impl<T> Stream for DownloadParquetRes<T>
where
  T: Stream<Item = reqwest::Result<Bytes>>,
{
  type Item = Result<Bytes>;

  fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    let project = self.project();
    let stream: Pin<&mut _> = project.stream;
    stream
      .poll_next(cx)
      .map(|a| a.map(|b| b.context(ReqwestClientSnafu)))
  }
}
