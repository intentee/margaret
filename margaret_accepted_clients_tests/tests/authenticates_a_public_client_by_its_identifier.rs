use margaret_accepted_clients::accepted_client_authentication::AcceptedClientAuthentication;
use margaret_accepted_clients::client_authentication_outcome::ClientAuthenticationOutcome;
use margaret_accepted_clients::presented_client_credentials::PresentedClientCredentials;
use margaret_accepted_clients_tests::accepted_clients_of::accepted_clients_of;
use margaret_accepted_clients_tests::fixture_client::fixture_client;

#[test]
fn authenticates_a_public_client_by_its_identifier() {
    let clients = accepted_clients_of(vec![fixture_client(
        "client:id",
        AcceptedClientAuthentication::Public,
    )])
    .expect("the client is accepted");

    assert!(matches!(
        clients.authenticate(&PresentedClientCredentials::of(None, Some("client:id"))),
        ClientAuthenticationOutcome::Authenticated(client) if client.client_id.as_str() == "client:id"
    ));
}
