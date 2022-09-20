use bytes::Bytes;
use std::net::SocketAddr;

use reqwest::StatusCode;

use crate::RaxiosHeaders;

#[derive(Debug)]
pub struct RaxiosResponse<T> {
    pub body: Option<T>,
    pub raw_body: Option<Bytes>,
    pub status: StatusCode,
    pub response_headers: RaxiosHeaders,
    pub remote_address: Option<SocketAddr>,
}
