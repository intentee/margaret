use std::sync::Arc;

use margaret_authorization_server_client::authorization_server_client::AuthorizationServerClient;
use margaret_authorization_server_client::server_unavailability::ServerUnavailability;
use margaret_authorization_server_client_tests::localhost_trust::localhost_trust;
use margaret_authorization_server_client_tests::oauth_client_declaration::OAuthClientDeclaration;
use margaret_authorization_server_client_tests::secret_basic_client::secret_basic_client;
use margaret_client_credentials::acquired_token::AcquiredToken;
use margaret_client_credentials::client_credentials::ClientCredentials;
use margaret_client_credentials_tests::artifact_store_target::artifact_store_target;
use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_jwt_verification_tests::token_trust_declaration::TokenTrustDeclaration;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

#[tokio::test]
async fn reports_an_unavailable_authorization_server() {
    let metadata = Arc::new(IssuerMetadata::awaiting());
    let client_credentials =
        ClientCredentials::create(Arc::new(AuthorizationServerClient::create(
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
        )));

    assert!(matches!(
        client_credentials
            .access_token(&artifact_store_target())
            .await,
        AcquiredToken::Unavailable(unavailability)
            if matches!(unavailability.as_ref(), ServerUnavailability::MetadataAwaited)
    ));
}
