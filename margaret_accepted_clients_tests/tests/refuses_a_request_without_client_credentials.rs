use std::sync::Arc;

use chrono::Utc;

use margaret_accepted_clients::client_authentication_outcome::ClientAuthenticationOutcome;
use margaret_accepted_clients::client_authentication_parameters::ClientAuthenticationParameters;
use margaret_accepted_clients::client_refusal::ClientRefusal;
use margaret_accepted_clients_tests::accepted_clients_of::accepted_clients_of;
use margaret_accepted_clients_tests::asserting_client::AssertingClient;
use margaret_accepted_clients_tests::fixture_client::fixture_client;
use margaret_accepted_clients_tests::unprivileged::UNPRIVILEGED;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_provider_state_storage::memory_provider_state::MemoryProviderState;
use margaret_registered_claims::numeric_date::NumericDate;

#[tokio::test]
async fn refuses_a_request_without_client_credentials() {
    let client = AssertingClient::holding_its_keys(fixture_client("portal"), UNPRIVILEGED);

    let clients = accepted_clients_of(vec![Arc::clone(&client.registered)]);

    assert!(matches!(
        clients
            .authenticate(
                &RequestAuthorization::Absent,
                &ClientAuthenticationParameters::default(),
                &MemoryProviderState::create(),
                NumericDate::from(Utc::now()),
            )
            .await,
        Ok(ClientAuthenticationOutcome::Refused(
            ClientRefusal::MissingCredentials
        ))
    ));
}
