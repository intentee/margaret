use margaret_accepted_clients::accepted_client_authentication::AcceptedClientAuthentication;
use margaret_accepted_clients::client_authentication_outcome::ClientAuthenticationOutcome;
use margaret_accepted_clients::client_refusal::ClientRefusal;
use margaret_accepted_clients::presented_client_credentials::PresentedClientCredentials;
use margaret_accepted_clients_tests::accepted_clients_of::accepted_clients_of;
use margaret_accepted_clients_tests::fixture_client::fixture_client;

#[test]
fn refuses_a_request_without_client_credentials() {
    let clients = accepted_clients_of(vec![fixture_client(
        "client:id",
        AcceptedClientAuthentication::ClientSecretBasic(
            "s3cret/+=".parse().expect("the secret is visible"),
        ),
    )])
    .expect("the client is accepted");

    assert!(matches!(
        clients.authenticate(&PresentedClientCredentials::of(None, None)),
        ClientAuthenticationOutcome::Refused(ClientRefusal::MissingCredentials)
    ));
}
