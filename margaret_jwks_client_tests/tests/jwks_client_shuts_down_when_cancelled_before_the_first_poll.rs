use std::sync::Arc;

use tokio_util::sync::CancellationToken;
use url::Url;

use margaret_jwks_client::JwksClient;
use margaret_jwks_client::static_issuer::StaticIssuer;
use margaret_jwks_client_tests::test_expected_claims::test_expected_claims;

#[tokio::test]
async fn jwks_client_shuts_down_when_cancelled_before_the_first_poll() {
    let cancellation_token = CancellationToken::new();

    cancellation_token.cancel();

    JwksClient::create(Arc::new(StaticIssuer::new(
        Url::parse("https://issuer.invalid/.well-known/jwks.json").expect("the url parses"),
        test_expected_claims(),
    )))
    .run(cancellation_token)
    .await
    .expect("a cancelled client shuts down cleanly");
}
