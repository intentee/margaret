use std::sync::Arc;

use margaret_authorization_server_client::authorization_server_client::AuthorizationServerClient;
use margaret_authorization_server_client_tests::constants::FIXTURE_CLIENT_ID;
use margaret_authorization_server_client_tests::fixture_client_secret::fixture_client_secret;
use margaret_authorization_server_client_tests::localhost_trust::localhost_trust;
use margaret_http::handles_head::HandlesHead;
use margaret_http_tests::fixture_request::FixtureRequest;
use margaret_issuer_key_set::issuer_key_set::IssuerKeySet;
use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_jwks_secret_store_tests::fixture_secrets::fixture_secrets;
use margaret_oidc_sign_in::sign_in_start_handler::SignInStartHandler;
use margaret_oidc_sign_in_tests::fixture_sign_in_flow::fixture_sign_in_flow;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

#[tokio::test]
async fn answers_a_sign_in_start_of_an_undiscovered_provider_as_unavailable() {
    let flow = fixture_sign_in_flow(
        Arc::new(AuthorizationServerClient::with_client_secret_basic(
            Arc::new(IssuerRequestClient::create().expect("the issuer request client builds")),
            Arc::new(IssuerMetadata::awaiting()),
            Arc::new(TrustedIssuer::polled(
                Arc::new(IssuerKeySet::awaiting()),
                localhost_trust(),
            )),
            FIXTURE_CLIENT_ID,
            fixture_client_secret(),
        )),
        fixture_secrets(),
    );

    assert!(matches!(
        SignInStartHandler::create(flow)
            .handle(&FixtureRequest::new(http::Method::GET, "/sign-in").into_request())
            .await,
        Ok(response) if response.status() == 503
    ));
}
