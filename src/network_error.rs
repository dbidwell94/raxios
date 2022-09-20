use std::{fmt::Display, net::SocketAddr};

use bytes::Bytes;
use reqwest::{Response, StatusCode};
use thiserror::Error;

/// Represents any non-200 HTTP status code
///
/// # Example
/// ```rust
/// use httpmock::prelude::*;
/// use raxios::Raxios;
///
/// #[tokio::main]
/// async fn main() {
///     let server = MockServer::start();
///     let raxios = Raxios::new(&server.base_url(), None).unwrap();
///
///     server.mock(|when, then| {
///         when.path("/test").method(GET);
///         then.status(400);
///     });
///
///     let response = raxios.get::<()>("/test", None).await;
///
///     if let Err(raxios::RaxiosError::NetworkError(ref err)) = response {
///         assert_eq!(400, err.status_code);
///         assert_eq!(Some(*server.address()), err.origin_address);
///         assert_eq!(false, err.raw_body.is_none());
///     } else {
///         panic!("Result was not an instance of NetworkError");
///     }
///
///     assert!(response.is_err());
/// }
/// ```
#[derive(Error, Debug)]
pub struct NetworkError {
    pub status_code: StatusCode,
    pub origin_address: Option<SocketAddr>,
    pub raw_body: Option<Bytes>,
}

impl Display for NetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let body = self
            .raw_body
            .as_ref()
            .map(|val| String::from_utf8(val.to_owned().to_vec()).ok())
            .unwrap_or_default()
            .unwrap_or_default();

        write!(
            f,
            "Error -- Status: {}, Origin: {}, Body: {}",
            self.status_code,
            self.origin_address
                .map(|socket| socket.to_string())
                .unwrap_or_default(),
            body
        )
    }
}

impl NetworkError {
    pub async fn new(response: Response) -> Self {
        Self {
            status_code: response.status(),
            origin_address: response.remote_addr(),
            raw_body: response.bytes().await.ok(),
        }
    }
}
