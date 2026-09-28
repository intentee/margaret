use std::sync::Arc;

use tokio_util::sync::CancellationToken;

use margaret_oidc_client::oidc_client::OidcClient;
use margaret_oidc_client_tests::localhost_trust::localhost_trust;

#[tokio::test]
async fn oidc_client_returns_when_cancelled_before_the_first_poll() {
    let cancellation_token = CancellationToken::new();

    cancellation_token.cancel();

    OidcClient::create(Arc::new(localhost_trust()))
        .run(cancellation_token)
        .await
        .expect("a cancelled client shuts down cleanly");
}
