use std::sync::Arc;

use margaret_authorization_server_client::authorization_server_client::AuthorizationServerClient;
use margaret_authorization_server_client_tests::fixture_client_id::FIXTURE_CLIENT_ID;
use margaret_authorization_server_client_tests::fixture_client_secret::fixture_client_secret;
use margaret_authorization_server_client_tests::localhost_trust::localhost_trust;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_http::token_admission::TokenAdmission;
use margaret_issuer_key_set::issuer_key_set::IssuerKeySet;
use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_token_introspection::introspect_bearer_token::introspect_bearer_token;
use margaret_token_introspection_tests::repository_claims::RepositoryClaims;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

#[tokio::test]
async fn refuses_while_the_authorization_server_is_unavailable() {
    let metadata = Arc::new(IssuerMetadata::awaiting());
    let server = AuthorizationServerClient::with_client_secret_basic(
        Arc::new(IssuerRequestClient::create().expect("the issuer request client builds")),
        Arc::clone(&metadata),
        Arc::new(TrustedIssuer::polled(
            Arc::new(IssuerKeySet::awaiting()),
            localhost_trust(),
        )),
        FIXTURE_CLIENT_ID,
        fixture_client_secret(),
    );

    assert!(matches!(
        introspect_bearer_token::<RepositoryClaims>(
            &RequestAuthorization::parse(Some("Bearer opaque-token")),
            &server,
        )
        .await,
        TokenAdmission::Refused(ResponseContinuation::Done(response)) if response.status() == 503
    ));
}
