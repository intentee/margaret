use margaret_accepted_clients::accepted_client_authentication::AcceptedClientAuthentication;
use margaret_accepted_clients::accepted_clients_error::AcceptedClientsError;
use margaret_accepted_clients::non_empty_set::NonEmptySet;
use margaret_accepted_clients_tests::accepted_clients_of::accepted_clients_of;
use margaret_accepted_clients_tests::fixture_client::fixture_client;

#[test]
fn rejects_a_client_naming_the_session_audience_as_a_resource() {
    let mut client = fixture_client("client:id", AcceptedClientAuthentication::Public);

    client.resources = NonEmptySet::of(
        "artifacts".parse().expect("the resource is an audience"),
        ["margaret".parse().expect("the audience is not empty")],
    );

    let clients = vec![client];

    assert!(matches!(
        accepted_clients_of(clients),
        Err(AcceptedClientsError::SessionAudienceResource { audience, .. }) if audience.as_str() == "margaret"
    ));
}
