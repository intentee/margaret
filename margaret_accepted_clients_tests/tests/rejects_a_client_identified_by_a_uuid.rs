use margaret_accepted_clients::accepted_client_authentication::AcceptedClientAuthentication;
use margaret_accepted_clients::accepted_clients_error::AcceptedClientsError;
use margaret_accepted_clients_tests::accepted_clients_of::accepted_clients_of;
use margaret_accepted_clients_tests::fixture_client::fixture_client;

#[test]
fn rejects_a_client_identified_by_a_uuid() {
    let clients = vec![fixture_client(
        "0b7f3c62-8a52-4c1e-9d4e-1f6f3a2b9c10",
        AcceptedClientAuthentication::Public,
    )];

    assert!(matches!(
        accepted_clients_of(clients),
        Err(AcceptedClientsError::UuidClientId { .. })
    ));
}
