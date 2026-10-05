use margaret_accepted_clients::accepted_client_authentication::AcceptedClientAuthentication;
use margaret_accepted_clients::client_authentication_outcome::ClientAuthenticationOutcome;
use margaret_accepted_clients::client_refusal::ClientRefusal;
use margaret_accepted_clients::presented_client_credentials::PresentedClientCredentials;
use margaret_accepted_clients_tests::accepted_clients_of::accepted_clients_of;
use margaret_accepted_clients_tests::fixture_client::fixture_client;
use margaret_http::request_authorization::RequestAuthorization;

#[test]
fn refuses_a_secret_from_a_public_client() {
    let clients = accepted_clients_of(vec![fixture_client(
        "client:id",
        AcceptedClientAuthentication::Public,
    )])
    .expect("the client is accepted");

    assert!(matches!(
        clients.authenticate(&PresentedClientCredentials::of(
            &RequestAuthorization::parse(Some("Basic Y2xpZW50JTNBaWQ6c2VjcmV0")),
            None
        )),
        ClientAuthenticationOutcome::Refused(ClientRefusal::UnexpectedSecret)
    ));
}
