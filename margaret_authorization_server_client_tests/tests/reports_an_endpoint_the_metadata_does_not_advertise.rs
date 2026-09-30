use std::sync::Arc;

use oauth2::EmptyExtraTokenFields;

use margaret_authorization_server_client::authorization_server_client::AuthorizationServerClient;
use margaret_authorization_server_client::endpoint_outcome::EndpointOutcome;
use margaret_authorization_server_client::server_endpoint::ServerEndpoint;
use margaret_authorization_server_client::server_unavailability::ServerUnavailability;
use margaret_authorization_server_client_tests::localhost_discovery_metadata::localhost_discovery_metadata;
use margaret_authorization_server_client_tests::localhost_trust::localhost_trust;
use margaret_authorization_server_client_tests::secret_basic_client::secret_basic_client;
use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_oidc_discovery::advertised_endpoint::AdvertisedEndpoint;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

#[tokio::test]
async fn reports_an_endpoint_the_metadata_does_not_advertise() {
    let metadata = Arc::new(IssuerMetadata::awaiting());

    metadata.hold(localhost_discovery_metadata(
        AdvertisedEndpoint::Unadvertised,
    ));

    let client = AuthorizationServerClient::create(
        Arc::new(IssuerRequestClient::create().expect("the issuer request client builds")),
        Arc::clone(&metadata),
        Arc::new(TrustedIssuer::for_oidc_issuer(
            metadata,
            Arc::new(localhost_trust()),
        )),
        Arc::new(secret_basic_client()),
    );

    assert!(matches!(
        client
            .introspect::<EmptyExtraTokenFields>("opaque-token")
            .await
            .expect("a secret basic client needs no assertion"),
        EndpointOutcome::Unavailable(ServerUnavailability::EndpointUnadvertised {
            endpoint: ServerEndpoint::Introspection
        })
    ));
}
