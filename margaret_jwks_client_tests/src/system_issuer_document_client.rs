use reqwest::Client;

use margaret_issuer_document_fetch::issuer_document_client::IssuerDocumentClient;

/// # Panics
///
/// Panics when the fixture it builds cannot be prepared.
#[must_use]
pub fn system_issuer_document_client() -> IssuerDocumentClient {
    IssuerDocumentClient::build(Client::builder()).expect("the issuer document client builds")
}
