use rustls::ClientConfig;
use url::Url;

use margaret_http_tests::mtls_fixture::MtlsFixture;
use margaret_jwks_client::JwksClientBundle;
use margaret_jwks_client::JwksClientBundleParams;
use margaret_jwks_client::jwks_client_error::JwksClientError;

#[test]
fn jwks_client_bundle_rejects_an_issuer_url_that_cannot_be_a_base() {
    let fixture = MtlsFixture::new();

    let Err(error) = JwksClientBundle::new(JwksClientBundleParams {
        client_config: ClientConfig::clone(&fixture.client_config),
        issuer_url: Url::parse("mailto:issuer@example.org").expect("the fixture url parses"),
    }) else {
        panic!("an issuer url that cannot be a base has no well known jwks path");
    };

    assert!(matches!(error, JwksClientError::IssuerUrlNotABase { .. }));
    assert_eq!(
        error.to_string(),
        "the issuer url 'mailto:issuer@example.org' cannot carry the well known jwks path: relative URL with a cannot-be-a-base base"
    );
}
