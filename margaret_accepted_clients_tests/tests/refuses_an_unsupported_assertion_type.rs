use std::sync::Arc;

use chrono::Utc;

use margaret_accepted_clients::client_authentication_outcome::ClientAuthenticationOutcome;
use margaret_accepted_clients::client_authentication_parameters::ClientAuthenticationParameters;
use margaret_accepted_clients::client_refusal::ClientRefusal;
use margaret_accepted_clients::registered_code_grant::RegisteredCodeGrant;
use margaret_accepted_clients_tests::accepted_clients_of::accepted_clients_of;
use margaret_accepted_clients_tests::asserting_client::AssertingClient;
use margaret_accepted_clients_tests::assertion_claims::assertion_claims;
use margaret_accepted_clients_tests::fixture_client::fixture_client;
use margaret_accepted_clients_tests::fixture_client_assertions::FixtureClientAssertions;
use margaret_accepted_clients_tests::unprivileged::UNPRIVILEGED;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_jwt_verification_tests::fixture_issuer::FIXTURE_ISSUER;
use margaret_registered_claims::numeric_date::NumericDate;

#[tokio::test]
async fn refuses_an_unsupported_assertion_type() {
    let client = AssertingClient::holding_its_keys(
        fixture_client("portal"),
        UNPRIVILEGED,
        Arc::new(FixtureClientAssertions::default()),
        RegisteredCodeGrant::Withheld,
    )
    .await;

    let clients = accepted_clients_of(vec![Arc::clone(&client.registered)]);

    assert!(matches!(
        clients
            .authenticate(
                &RequestAuthorization::Absent,
                &ClientAuthenticationParameters {
                    client_assertion: Some(
                        client.assertion(&assertion_claims("portal", FIXTURE_ISSUER))
                    ),
                    client_assertion_type: Some(
                        "urn:ietf:params:oauth:client-assertion-type:saml2-bearer".to_string(),
                    ),
                    client_id: None,
                },
                NumericDate::from(Utc::now()),
            )
            .await,
        Ok(ClientAuthenticationOutcome::Refused(
            ClientRefusal::UnsupportedAssertionType
        ))
    ));
}
