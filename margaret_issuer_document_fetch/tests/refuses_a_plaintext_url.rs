use std::time::Duration;

use reqwest::Client;
use tokio_util::sync::CancellationToken;
use url::Url;

use margaret_issuer_document_fetch::issuer_document_client::IssuerDocumentClient;
use margaret_issuer_document_fetch::issuer_document_fetch::IssuerDocumentFetch;
use margaret_issuer_document_fetch::issuer_document_request::IssuerDocumentRequest;

#[tokio::test]
async fn refuses_a_plaintext_url_before_connecting() {
    let client =
        IssuerDocumentClient::build(Client::builder()).expect("the issuer document client builds");

    let fetched = client
        .fetch(IssuerDocumentRequest {
            cancellation_token: &CancellationToken::new(),
            timeout: Duration::MAX,
            url: Url::parse("http://localhost/document").expect("the plaintext url parses"),
        })
        .await;

    assert!(matches!(
        fetched,
        IssuerDocumentFetch::TransportFailed(error) if error.is_builder()
    ));
}
