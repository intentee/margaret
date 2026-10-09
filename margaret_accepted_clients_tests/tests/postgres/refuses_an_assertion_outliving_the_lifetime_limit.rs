use std::sync::Arc;
use std::time::Duration;

use chrono::Utc;
use serde_json::json;

use margaret_accepted_clients::client_authentication_outcome::ClientAuthenticationOutcome;
use margaret_accepted_clients::client_refusal::ClientRefusal;
use margaret_accepted_clients_tests::accepted_clients_of::accepted_clients_of;
use margaret_accepted_clients_tests::asserted_parameters::asserted_parameters;
use margaret_accepted_clients_tests::asserting_client::AssertingClient;
use margaret_accepted_clients_tests::assertion_claims::assertion_claims;
use margaret_accepted_clients_tests::fixture_client::fixture_client;
use margaret_accepted_clients_tests::unprivileged::UNPRIVILEGED;
use margaret_client_assertions_tests::started_with_client_assertions::started_with_client_assertions;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_jwt_verification_tests::fixture_issuer::FIXTURE_ISSUER;
use margaret_registered_claims::numeric_date::NumericDate;

#[tokio::test]
async fn refuses_an_assertion_outliving_the_lifetime_limit() {
    let started = started_with_client_assertions().await;
    let client = AssertingClient::holding_its_keys(
        fixture_client("portal"),
        UNPRIVILEGED,
        Arc::clone(&started.database),
    );
    let mut claims = assertion_claims("portal", FIXTURE_ISSUER);

    claims["exp"] = json!(
        NumericDate::from(Utc::now())
            .after(Duration::from_hours(2))
            .seconds_since_epoch()
    );

    let clients = accepted_clients_of(vec![Arc::clone(&client.registered)]);

    assert!(matches!(
        clients
            .authenticate(
                &RequestAuthorization::Absent,
                &asserted_parameters(client.assertion(&claims)),
                NumericDate::from(Utc::now()),
            )
            .await,
        Ok(ClientAuthenticationOutcome::Refused(
            ClientRefusal::AssertionOutlivesLimit { .. }
        ))
    ));
}
