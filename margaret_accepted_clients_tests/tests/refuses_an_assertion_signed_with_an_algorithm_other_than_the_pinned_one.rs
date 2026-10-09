use std::sync::Arc;

use chrono::Utc;

use margaret_accepted_clients::client_authentication_outcome::ClientAuthenticationOutcome;
use margaret_accepted_clients::client_refusal::ClientRefusal;
use margaret_accepted_clients::registered_code_grant::RegisteredCodeGrant;
use margaret_accepted_clients_tests::accepted_clients_of::accepted_clients_of;
use margaret_accepted_clients_tests::asserted_parameters::asserted_parameters;
use margaret_accepted_clients_tests::asserting_client::AssertingClient;
use margaret_accepted_clients_tests::assertion_claims::assertion_claims;
use margaret_accepted_clients_tests::fixture_client::fixture_client;
use margaret_accepted_clients_tests::fixture_client_assertions::FixtureClientAssertions;
use margaret_accepted_clients_tests::unprivileged::UNPRIVILEGED;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_jwt_verification_tests::fixture_issuer::FIXTURE_ISSUER;
use margaret_registered_claims::numeric_date::NumericDate;

#[tokio::test]
async fn refuses_an_assertion_signed_with_an_algorithm_other_than_the_pinned_one() {
    let client = AssertingClient::holding_its_keys(
        fixture_client("portal"),
        UNPRIVILEGED,
        Arc::new(FixtureClientAssertions::default()),
        RegisteredCodeGrant::Withheld,
    )
    .await;
    let rs256_assertion = client
        .roller
        .jwks_secret_holder()
        .get()
        .rsa()
        .current()
        .sign_jwt(&assertion_claims("portal", FIXTURE_ISSUER))
        .expect("the rsa key signs the assertion");

    assert!(matches!(
        accepted_clients_of(vec![Arc::clone(&client.registered)])
            .authenticate(
                &RequestAuthorization::Absent,
                &asserted_parameters(rs256_assertion),
                NumericDate::from(Utc::now()),
            )
            .await,
        Ok(ClientAuthenticationOutcome::Refused(
            ClientRefusal::AssertionRejected(JwtRejection::AlgorithmNotPinned {
                pinned: JwsAlgorithm::Es256
            })
        ))
    ));
}
