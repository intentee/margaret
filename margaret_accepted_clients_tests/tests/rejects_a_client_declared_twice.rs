use margaret_accepted_clients::accepted_client_authentication::AcceptedClientAuthentication;
use margaret_accepted_clients::accepted_clients_error::AcceptedClientsError;
use margaret_accepted_clients_tests::accepted_clients_of::accepted_clients_of;
use margaret_accepted_clients_tests::fixture_client::fixture_client;

#[test]
fn rejects_a_client_declared_twice() {
    let clients = vec![
        fixture_client("client:id", AcceptedClientAuthentication::Public),
        fixture_client("client:id", AcceptedClientAuthentication::Public),
    ];

    assert!(matches!(
        accepted_clients_of(clients),
        Err(AcceptedClientsError::DuplicateClientId { client_id }) if client_id.as_str() == "client:id"
    ));
}
