use std::net::SocketAddr;

use reqwest::StatusCode;

use crate::RaxiosHeaders;

#[derive(Debug)]
pub struct RaxiosResponse<T> {
    pub body: T,
    pub status: StatusCode,
    pub headers: RaxiosHeaders,
    pub remote_address: Option<SocketAddr>,
}
