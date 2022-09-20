use std::{fmt::Display, net::SocketAddr};

use reqwest::{Response, StatusCode};
use thiserror::Error;

#[derive(Error, Debug)]
/// Represents any non-200 HTTP status code
/// 
/// # Example
/// ```rust
/// use httpmock::prelude::*;
/// use raxios::Raxios;
/// 
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let server = MockServer::start();
///     let raxios = Raxios::new(&server.base_url(), None)?;
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
///     } else {
///         panic!("Result was not an instance of NetworkError");
///     }
/// 
///     assert!(response.is_err());
/// 
///     Ok(())
/// }
/// ```
pub struct NetworkError {
    pub status_code: StatusCode,
    pub origin_address: Option<SocketAddr>,
}

impl Display for NetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error -- Status: {}, Origin: {}",
            self.status_code,
            self.origin_address
                .map(|socket| socket.to_string())
                .unwrap_or_default()
        )
    }
}

impl From<Response> for NetworkError {
    fn from(res: Response) -> Self {
        Self::new(res)
    }
}

impl NetworkError {
    fn new(response: Response) -> Self {
        Self {
            status_code: response.status(),
            origin_address: response.remote_addr(),
        }
    }
}
