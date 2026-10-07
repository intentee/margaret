use std::sync::Arc;

use chrono::Utc;

use margaret_accepted_clients::client_authentication_outcome::ClientAuthenticationOutcome;
use margaret_accepted_clients::client_refusal::ClientRefusal;
use margaret_accepted_clients_tests::accepted_clients_of::accepted_clients_of;
use margaret_accepted_clients_tests::asserted_parameters::asserted_parameters;
use margaret_accepted_clients_tests::asserting_client::AssertingClient;
use margaret_accepted_clients_tests::assertion_claims::assertion_claims;
use margaret_accepted_clients_tests::fixture_client::fixture_client;
use margaret_accepted_clients_tests::unprivileged::UNPRIVILEGED;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_jwt_verification_tests::fixture_issuer::FIXTURE_ISSUER;
use margaret_provider_state_storage::assertion_refusal::AssertionRefusal;
use margaret_provider_state_storage::memory_provider_state::MemoryProviderState;
use margaret_registered_claims::numeric_date::NumericDate;

#[tokio::test]
async fn refuses_a_replayed_client_assertion() {
    let client = AssertingClient::holding_its_keys(fixture_client("portal"), UNPRIVILEGED);
    let clients = accepted_clients_of(vec![Arc::clone(&client.registered)]);
    let parameters =
        asserted_parameters(client.assertion(&assertion_claims("portal", FIXTURE_ISSUER)));
    let state = MemoryProviderState::create();
    let now = NumericDate::from(Utc::now());

    clients
        .authenticate(&RequestAuthorization::Absent, &parameters, &state, now)
        .await
        .expect("the first presentation spends the assertion");

    assert!(matches!(
        clients
            .authenticate(&RequestAuthorization::Absent, &parameters, &state, now)
            .await,
        Ok(ClientAuthenticationOutcome::Refused(
            ClientRefusal::AssertionRefused(AssertionRefusal::Replayed)
        ))
    ));
}
