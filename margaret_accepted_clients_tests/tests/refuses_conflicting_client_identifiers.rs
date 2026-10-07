use std::sync::Arc;

use chrono::Utc;

use margaret_accepted_clients::client_authentication_outcome::ClientAuthenticationOutcome;
use margaret_accepted_clients::client_authentication_parameters::ClientAuthenticationParameters;
use margaret_accepted_clients::client_refusal::ClientRefusal;
use margaret_accepted_clients_tests::accepted_clients_of::accepted_clients_of;
use margaret_accepted_clients_tests::asserting_client::AssertingClient;
use margaret_accepted_clients_tests::assertion_claims::assertion_claims;
use margaret_accepted_clients_tests::fixture_client::fixture_client;
use margaret_accepted_clients_tests::unprivileged::UNPRIVILEGED;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_jwt_verification_tests::fixture_issuer::FIXTURE_ISSUER;
use margaret_oauth_vocabulary::jwt_bearer_client_assertion_type::JWT_BEARER_CLIENT_ASSERTION_TYPE;
use margaret_provider_state_storage::memory_provider_state::MemoryProviderState;
use margaret_registered_claims::numeric_date::NumericDate;

#[tokio::test]
async fn refuses_conflicting_client_identifiers() {
    let client = AssertingClient::holding_its_keys(fixture_client("portal"), UNPRIVILEGED);

    let clients = accepted_clients_of(vec![Arc::clone(&client.registered)]);

    assert!(matches!(
        clients
            .authenticate(
                &RequestAuthorization::Absent,
                &ClientAuthenticationParameters {
                    client_assertion: Some(
                        client.assertion(&assertion_claims("portal", FIXTURE_ISSUER))
                    ),
                    client_assertion_type: Some(JWT_BEARER_CLIENT_ASSERTION_TYPE.to_string()),
                    client_id: Some("blog".to_string()),
                },
                &MemoryProviderState::create(),
                NumericDate::from(Utc::now()),
            )
            .await,
        Ok(ClientAuthenticationOutcome::Refused(
            ClientRefusal::ConflictingClientIds
        ))
    ));
}
