use std::sync::Arc;

use margaret_authorization_server_client::authorization_server_client::AuthorizationServerClient;
use margaret_authorization_server_client_tests::localhost_trust::localhost_trust;
use margaret_authorization_server_client_tests::oauth_client_declaration::OAuthClientDeclaration;
use margaret_authorization_server_client_tests::secret_basic_client::secret_basic_client;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_http::token_admission::TokenAdmission;
use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_jwt_verification_tests::token_trust_declaration::TokenTrustDeclaration;
use margaret_token_introspection::introspect_bearer_token::introspect_bearer_token;
use margaret_token_introspection_tests::repository_claims::RepositoryClaims;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

#[tokio::test]
async fn refuses_while_the_authorization_server_is_unavailable() {
    let metadata = Arc::new(IssuerMetadata::awaiting());
    let server = AuthorizationServerClient::create(
        Arc::new(IssuerRequestClient::create().expect("the issuer request client builds")),
        Arc::clone(&metadata),
        Arc::new(TrustedIssuer::for_oidc_issuer(
            metadata,
            Arc::new(TokenTrustDeclaration {
                trust: localhost_trust(),
            }),
        )),
        Arc::new(OAuthClientDeclaration {
            client: secret_basic_client(),
        }),
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
