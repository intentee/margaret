use std::collections::BTreeSet;
use std::sync::Arc;

use margaret_authorization_server_client::authorization_server_client::AuthorizationServerClient;
use margaret_authorization_server_client::endpoint_outcome::EndpointOutcome;
use margaret_authorization_server_client::server_unavailability::ServerUnavailability;
use margaret_authorization_server_client::target_audience::TargetAudience;
use margaret_authorization_server_client::token_target::TokenTarget;
use margaret_authorization_server_client_tests::fixture_client_id::FIXTURE_CLIENT_ID;
use margaret_authorization_server_client_tests::fixture_client_secret::fixture_client_secret;
use margaret_authorization_server_client_tests::localhost_trust::localhost_trust;
use margaret_issuer_key_set::issuer_key_set::IssuerKeySet;
use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

#[tokio::test]
async fn reports_metadata_that_has_not_been_discovered() {
    let metadata = Arc::new(IssuerMetadata::awaiting());
    let client = AuthorizationServerClient::with_client_secret_basic(
        Arc::new(IssuerRequestClient::create().expect("the issuer request client builds")),
        Arc::clone(&metadata),
        Arc::new(TrustedIssuer::create(
            Arc::new(IssuerKeySet::awaiting()),
            localhost_trust(),
        )),
        FIXTURE_CLIENT_ID,
        fixture_client_secret(),
    );

    assert!(matches!(
        client
            .client_credentials(&TokenTarget {
                audience: TargetAudience::Unspecified,
                scopes: BTreeSet::new(),
            })
            .await,
        EndpointOutcome::Unavailable(ServerUnavailability::MetadataAwaited)
    ));
}
