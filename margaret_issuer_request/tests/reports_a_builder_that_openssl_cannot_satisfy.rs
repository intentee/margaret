use reqwest::Client;
use reqwest::tls::Version;

use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_issuer_request::issuer_request_error::IssuerRequestError;

#[test]
fn reports_a_builder_that_openssl_cannot_satisfy() {
    let requires_tls_1_3 = Client::builder().min_tls_version(Version::TLS_1_3);

    assert!(matches!(
        IssuerRequestClient::build(requires_tls_1_3),
        Err(IssuerRequestError::ClientBuild { .. })
    ));
}
