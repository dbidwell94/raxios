use thiserror::Error;

use crate::network_error::NetworkError;

pub type RaxiosResult<T> = Result<T, RaxiosError>;

#[derive(Error, Debug)]
pub enum RaxiosError {
    #[error(transparent)]
    Unknown(anyhow::Error),
    #[error("Unable to parse header: {0} => {1}")]
    HeaderParseError(String, String),
    #[error("{0} is not a valid Url")]
    InvalidUrl(String),
    #[error("Request failed. StatusCode: {:?}", err.status())]
    UnableToSendRequest { err: reqwest::Error },
    #[error(transparent)]
    NetworkError(NetworkError),
    #[error(transparent)]
    SerializationError(serde_json::Error),
}
