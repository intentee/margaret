use std::sync::Arc;

use chrono::Utc;

use margaret_accepted_clients::accepted_clients_error::AcceptedClientsError;
use margaret_accepted_clients_tests::accepted_clients_of::accepted_clients_of;
use margaret_accepted_clients_tests::asserted_parameters::asserted_parameters;
use margaret_accepted_clients_tests::asserting_client::AssertingClient;
use margaret_accepted_clients_tests::assertion_claims::assertion_claims;
use margaret_accepted_clients_tests::fixture_client::fixture_client;
use margaret_accepted_clients_tests::unprivileged::UNPRIVILEGED;
use margaret_client_assertions_tests::started_with_client_assertions::started_with_client_assertions;
use margaret_database_tests::table_privilege::TablePrivilege;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_jwt_verification_tests::fixture_issuer::FIXTURE_ISSUER;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_sql_identifier::table_namespace::TableNamespace;

#[tokio::test]
async fn reports_an_assertion_that_cannot_be_remembered() {
    let started = started_with_client_assertions().await;

    started
        .administration
        .revoke(
            TablePrivilege::Insert,
            TableNamespace::Framework,
            "client_assertions",
        )
        .await;

    let client = AssertingClient::holding_its_keys(
        fixture_client("portal"),
        UNPRIVILEGED,
        Arc::clone(&started.database),
    );
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
