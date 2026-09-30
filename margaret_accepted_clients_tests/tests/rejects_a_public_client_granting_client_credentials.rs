use margaret_accepted_clients::accepted_client_authentication::AcceptedClientAuthentication;
use margaret_accepted_clients::accepted_clients_error::AcceptedClientsError;
use margaret_accepted_clients::grant_type::GrantType;
use margaret_accepted_clients_tests::accepted_clients_of::accepted_clients_of;
use margaret_accepted_clients_tests::fixture_client::fixture_client;

#[test]
fn rejects_a_public_client_granting_client_credentials() {
    let mut client = fixture_client("client:id", AcceptedClientAuthentication::Public);

    client.grants.insert(GrantType::ClientCredentials);

    let clients = vec![client];

    assert!(matches!(
        accepted_clients_of(clients),
        Err(AcceptedClientsError::PublicClientCredentials { .. })
    ));
}
