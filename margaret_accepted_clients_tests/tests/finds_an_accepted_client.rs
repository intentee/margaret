use margaret_accepted_clients::accepted_client_authentication::AcceptedClientAuthentication;
use margaret_accepted_clients_tests::accepted_clients_of::accepted_clients_of;
use margaret_accepted_clients_tests::fixture_client::fixture_client;

#[test]
fn finds_an_accepted_client() {
    let clients = accepted_clients_of(vec![fixture_client(
        "client:id",
        AcceptedClientAuthentication::Public,
    )])
    .expect("the client is accepted");

    assert!(
        clients
            .find("client:id")
            .is_some_and(|client| client.client_id.as_str() == "client:id")
    );
    assert!(clients.find("other").is_none());
}
