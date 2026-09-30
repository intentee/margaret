use std::collections::BTreeSet;
use std::sync::Arc;

use url::Url;

use margaret_authorization_server_client::authorization_server_client::AuthorizationServerClient;
use margaret_authorization_server_client::server_unavailability::ServerUnavailability;
use margaret_authorization_server_client_tests::localhost_trust::localhost_trust;
use margaret_authorization_server_client_tests::secret_basic_client::secret_basic_client;
use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_jwks_secret_store_tests::rolled_store::rolled_store;
use margaret_oidc_sign_in::sign_in_beginning::SignInBeginning;
use margaret_oidc_sign_in::sign_in_flow::SignInFlow;
use margaret_oidc_sign_in::sign_in_request::SignInRequest;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

#[tokio::test]
async fn reports_undiscovered_metadata_when_beginning() {
    let metadata = Arc::new(IssuerMetadata::awaiting());
    let flow = SignInFlow::create(
        Arc::new(AuthorizationServerClient::create(
            Arc::new(IssuerRequestClient::create().expect("the issuer request client builds")),
            Arc::clone(&metadata),
            Arc::new(TrustedIssuer::for_oidc_issuer(
                metadata,
                Arc::new(localhost_trust()),
            )),
            Arc::new(secret_basic_client()),
        )),
        Arc::new(rolled_store(fresh_p256_secret())),
    );

    assert!(matches!(
        flow.begin(SignInRequest {
            callback: Url::parse("https://client.example/callback").expect("the callback is a url"),
            scopes: BTreeSet::new(),
        })
        .await
        .expect("no transaction is signed"),
        SignInBeginning::Unavailable(ServerUnavailability::MetadataAwaited)
    ));
}
