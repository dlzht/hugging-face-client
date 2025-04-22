use snafu::{Location, Snafu};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
  #[snafu(display("Failed to receive response from Hugging Face: {}", message))]
  HuggingFaceResponse { message: String },

  #[snafu(display("Failed to get env variable"))]
  EnvVariable {
    #[snafu(source)]
    source: std::env::VarError,
    #[snafu(implicit)]
    location: Location,

    variable: String,
  },

  #[snafu(display("Failed to process http request"))]
  ReqwestClient {
    #[snafu(source)]
    source: reqwest::Error,
    #[snafu(implicit)]
    location: Location,
  },

  #[snafu(display("Failed to serialize JSON"))]
  SerializeJson {
    #[snafu(source)]
    source: serde_json::Error,
    #[snafu(implicit)]
    location: Location,
  },

  #[snafu(display("Failed to deserialize JSON"))]
  DeserializeJson {
    #[snafu(source)]
    source: serde_json::Error,
    #[snafu(implicit)]
    location: Location,
  },

  #[snafu(display("{}", message))]
  PlainMessage {
    message: String,
    #[snafu(implicit)]
    location: Location,
  },

  #[snafu(display("Impossible error!"))]
  Impossible {
    #[snafu(implicit)]
    location: Location,
  },
}
