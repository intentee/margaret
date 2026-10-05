use margaret_accepted_clients::client_authentication_outcome::ClientAuthenticationOutcome;
use margaret_accepted_clients::client_refusal::ClientRefusal;
use margaret_accepted_clients::presented_client_credentials::PresentedClientCredentials;
use margaret_accepted_clients_tests::accepted_clients_of::accepted_clients_of;
use margaret_accepted_clients_tests::confidential_authentication::confidential_authentication;
use margaret_accepted_clients_tests::fixture_client::fixture_client;
use margaret_http::request_authorization::RequestAuthorization;

#[test]
fn refuses_a_wrong_client_secret() {
    let clients = accepted_clients_of(vec![fixture_client(
        "client:id",
        confidential_authentication("s3cret/+="),
    )])
    .expect("the client is accepted");

    assert!(matches!(
        clients.authenticate(&PresentedClientCredentials::of(
            &RequestAuthorization::parse(Some("Basic Y2xpZW50JTNBaWQ6d3Jvbmc=")),
            None
        )),
        ClientAuthenticationOutcome::Refused(ClientRefusal::WrongSecret)
    ));
}
