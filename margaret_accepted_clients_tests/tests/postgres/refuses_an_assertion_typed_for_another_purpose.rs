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
use margaret_client_assertions_tests::started_with_client_assertions::started_with_client_assertions;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_jwt_verification_tests::fixture_issuer::FIXTURE_ISSUER;
use margaret_registered_claims::numeric_date::NumericDate;

#[tokio::test]
async fn refuses_an_assertion_typed_for_another_purpose() {
    let started = started_with_client_assertions().await;
    let client = AssertingClient::holding_its_keys(
        fixture_client("portal"),
        UNPRIVILEGED,
        Arc::clone(&started.database),
    );
    let access_token = client.secrets.get().current().sign_json(
        &assertion_claims("portal", FIXTURE_ISSUER),
        JwtType::AccessToken,
    );
    let clients = accepted_clients_of(vec![Arc::clone(&client.registered)]);

    assert!(matches!(
        clients
            .authenticate(
                &RequestAuthorization::Absent,
                &asserted_parameters(access_token),
                NumericDate::from(Utc::now()),
            )
            .await,
        Ok(ClientAuthenticationOutcome::Refused(
            ClientRefusal::AssertionRejected(JwtRejection::Type(_))
        ))
    ));
}
