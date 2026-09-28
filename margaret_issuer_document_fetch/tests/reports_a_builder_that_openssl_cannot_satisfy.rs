use reqwest::Client;
use reqwest::tls::Version;

use margaret_issuer_document_fetch::issuer_document_client::IssuerDocumentClient;
use margaret_issuer_document_fetch::issuer_document_fetch_error::IssuerDocumentFetchError;

#[test]
fn reports_a_builder_that_openssl_cannot_satisfy() {
    let requires_tls_1_3 = Client::builder().min_tls_version(Version::TLS_1_3);

    assert!(matches!(
        IssuerDocumentClient::build(requires_tls_1_3),
        Err(IssuerDocumentFetchError::ClientBuild { .. })
    ));
}
