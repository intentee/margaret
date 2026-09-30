use tokio_util::sync::CancellationToken;
use url::Url;

use margaret_issuer_request::issuer_document::IssuerDocument;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;

#[tokio::test]
async fn refuses_a_plaintext_url_before_connecting() {
    let client = IssuerRequestClient::create().expect("the issuer request client builds");

    let fetched = client
        .fetch_document(
            Url::parse("http://localhost/document").expect("the plaintext url parses"),
            &CancellationToken::new(),
        )
        .await;

    assert!(matches!(
        fetched,
        IssuerDocument::TransportFailed(error) if error.is_builder()
    ));
}
