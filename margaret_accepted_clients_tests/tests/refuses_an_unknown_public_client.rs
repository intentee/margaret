use std::sync::Arc;

use chrono::Utc;

use margaret_accepted_clients::client_authentication_outcome::ClientAuthenticationOutcome;
use margaret_accepted_clients::client_authentication_parameters::ClientAuthenticationParameters;
use margaret_accepted_clients::client_refusal::ClientRefusal;
use margaret_accepted_clients::registered_client::RegisteredClient;
use margaret_accepted_clients_tests::accepted_clients_of::accepted_clients_of;
use margaret_accepted_clients_tests::fixture_client::fixture_client;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_registered_claims::numeric_date::NumericDate;

#[tokio::test]
async fn refuses_an_unknown_public_client() {
    let clients = accepted_clients_of(vec![Arc::new(RegisteredClient::public(fixture_client(
        "blog",
    )))]);

    assert!(matches!(
        clients
            .authenticate(
                &RequestAuthorization::Absent,
                &ClientAuthenticationParameters {
                    client_id: Some("portal".to_string()),
                    ..ClientAuthenticationParameters::default()
                },
                NumericDate::from(Utc::now()),
            )
            .await,
        Ok(ClientAuthenticationOutcome::Refused(
            ClientRefusal::UnknownClient
        ))
    ));
}
