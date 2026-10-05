use std::sync::Arc;

use oauth2::AccessToken;
use serde_json::Value;

use margaret_authorization_server_client::authorization_server_client::AuthorizationServerClient;
use margaret_authorization_server_client::server_unavailability::ServerUnavailability;
use margaret_authorization_server_client::userinfo_outcome::UserinfoOutcome;
use margaret_authorization_server_client_tests::localhost_trust::localhost_trust;
use margaret_authorization_server_client_tests::oauth_client_declaration::OAuthClientDeclaration;
use margaret_authorization_server_client_tests::secret_basic_client::secret_basic_client;
use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_jwt_verification_tests::token_trust_declaration::TokenTrustDeclaration;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

#[tokio::test]
async fn reports_undiscovered_metadata_for_userinfo() {
    let metadata = Arc::new(IssuerMetadata::awaiting());
    let client = AuthorizationServerClient::create(
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
        client
            .userinfo::<Value>(&AccessToken::new("token".to_string()))
            .await,
        UserinfoOutcome::Unavailable(ServerUnavailability::MetadataAwaited)
    ));
}
