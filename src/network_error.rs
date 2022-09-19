use std::{fmt::Display, net::SocketAddr};

use reqwest::{Response, StatusCode};
use thiserror::Error;

#[derive(Error, Debug)]
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
