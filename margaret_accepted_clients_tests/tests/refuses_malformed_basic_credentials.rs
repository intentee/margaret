use margaret_accepted_clients::accepted_client_authentication::AcceptedClientAuthentication;
use margaret_accepted_clients::client_authentication_outcome::ClientAuthenticationOutcome;
use margaret_accepted_clients::client_refusal::ClientRefusal;
use margaret_accepted_clients::presented_client_credentials::PresentedClientCredentials;
use margaret_accepted_clients_tests::accepted_clients_of::accepted_clients_of;
use margaret_accepted_clients_tests::fixture_client::fixture_client;
use margaret_http::request_authorization::RequestAuthorization;

#[test]
fn refuses_malformed_basic_credentials() {
    let clients = accepted_clients_of(vec![fixture_client(
        "client:id",
        AcceptedClientAuthentication::Public,
    )])
    .expect("the client is accepted");

    for authorization in [
        "Basic line\nbreak",
        "Bearer token",
        "Basic JUZGOnNlY3JldA==",
        "Basic Y2xpZW50OiVGRg==",
    ] {
        assert!(matches!(
            clients.authenticate(&PresentedClientCredentials::of(
                &RequestAuthorization::parse(Some(authorization)),
                None
            )),
            ClientAuthenticationOutcome::Refused(ClientRefusal::MalformedCredentials)
        ));
    }
}
