use margaret_accepted_clients::accepted_client_authentication::AcceptedClientAuthentication;
use margaret_accepted_clients::accepted_clients_error::AcceptedClientsError;
use margaret_accepted_clients_tests::accepted_clients_of::accepted_clients_of;
use margaret_accepted_clients_tests::fixture_client::fixture_client;

#[test]
fn rejects_a_client_granting_codes_without_a_redirect_uri() {
    let mut client = fixture_client("client:id", AcceptedClientAuthentication::Public);

    client.redirect_uris.clear();

    let clients = vec![client];

    assert!(matches!(
        accepted_clients_of(clients),
        Err(AcceptedClientsError::WithoutRedirectUri { .. })
    ));
}
