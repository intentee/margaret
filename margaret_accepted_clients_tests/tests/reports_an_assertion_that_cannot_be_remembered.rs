use std::sync::Arc;

use chrono::Utc;

use margaret_accepted_clients::accepted_clients_error::AcceptedClientsError;
use margaret_accepted_clients::registered_code_grant::RegisteredCodeGrant;
use margaret_accepted_clients_tests::accepted_clients_of::accepted_clients_of;
use margaret_accepted_clients_tests::asserted_parameters::asserted_parameters;
use margaret_accepted_clients_tests::asserting_client::AssertingClient;
use margaret_accepted_clients_tests::assertion_claims::assertion_claims;
use margaret_accepted_clients_tests::fixture_client::fixture_client;
use margaret_accepted_clients_tests::unprivileged::UNPRIVILEGED;
use margaret_accepted_clients_tests::unreachable_client_assertions::UnreachableClientAssertions;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_jwt_verification_tests::fixture_issuer::FIXTURE_ISSUER;
use margaret_registered_claims::numeric_date::NumericDate;

#[tokio::test]
async fn reports_an_assertion_that_cannot_be_remembered() {
    let client = AssertingClient::holding_its_keys(
        fixture_client("portal"),
        UNPRIVILEGED,
        Arc::new(UnreachableClientAssertions),
        RegisteredCodeGrant::Withheld,
    )
    .await;
    let clients = accepted_clients_of(vec![Arc::clone(&client.registered)]);

    assert!(matches!(
        clients
            .authenticate(
                &RequestAuthorization::Absent,
                &asserted_parameters(client.assertion(&assertion_claims("portal", FIXTURE_ISSUER))),
                NumericDate::from(Utc::now()),
            )
            .await,
        Err(AcceptedClientsError::RememberClientAssertion {
            client_id: "portal",
            ..
        })
    ));
}
