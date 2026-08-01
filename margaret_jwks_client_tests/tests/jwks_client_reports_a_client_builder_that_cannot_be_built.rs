use std::sync::Arc;

use reqwest::Client;
use tokio_util::sync::CancellationToken;
use url::Url;

use margaret_jwks_client::JwksClient;
use margaret_jwks_client::static_issuer::StaticIssuer;
use margaret_jwks_client_tests::test_expected_claims::test_expected_claims;

#[tokio::test]
async fn jwks_client_reports_a_client_builder_that_cannot_be_built() {
    let broken_builder = Client::builder().use_preconfigured_tls(0u8);

    assert!(
        JwksClient::create(Arc::new(StaticIssuer::new(
            Url::parse("https://issuer.invalid/.well-known/jwks.json").expect("the url parses"),
            test_expected_claims(),
        )))
        .run_with_client_builder(broken_builder, CancellationToken::new())
        .await
        .is_err()
    );
}
