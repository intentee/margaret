use bytes::Bytes;
use reqwest::StatusCode;

#[derive(Debug)]
pub enum IssuerDocumentFetch {
    Cancelled,
    Fetched(Bytes),
    TransportFailed(reqwest::Error),
    UnexpectedStatus(StatusCode),
}
