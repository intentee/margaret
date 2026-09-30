use bytes::Bytes;
use reqwest::StatusCode;

#[derive(Debug)]
pub enum IssuerDocument {
    Cancelled,
    Fetched(Bytes),
    Oversized { max_bytes: usize },
    TransportFailed(reqwest::Error),
    UnexpectedStatus(StatusCode),
}
