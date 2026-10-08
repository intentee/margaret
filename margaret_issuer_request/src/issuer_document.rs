use bytes::Bytes;
use reqwest::StatusCode;

use crate::issuer_exchange_error::IssuerExchangeError;

#[derive(Debug)]
pub enum IssuerDocument {
    Cancelled,
    Failed(IssuerExchangeError),
    Fetched(Bytes),
    Oversized { max_bytes: usize },
    UnexpectedStatus(StatusCode),
}
