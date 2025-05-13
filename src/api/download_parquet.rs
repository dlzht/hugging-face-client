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
pub struct DownloadParquetReq<'a> {
  #[serde(rename = "repo_id")]
  pub(crate) repo_name: &'a str,

  pub(crate) subset: &'a str,

  pub(crate) split: &'a str,

  pub(crate) nth: usize,
}

impl<'a> DownloadParquetReq<'a> {
  pub fn new(repo_name: &'a str, subset: &'a str, split: &'a str) -> DownloadParquetReq<'a> {
    DownloadParquetReq {
      repo_name,
      subset,
      split,
      nth: 0,
    }
  }

  pub fn nth(mut self, nth: usize) -> DownloadParquetReq<'a> {
    self.nth = nth;
    self
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
