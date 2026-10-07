use std::collections::BTreeMap;
use std::sync::Arc;

use margaret_authorization_server_client::authorization_server_client::AuthorizationServerClient;
use margaret_authorization_server_client_tests::fixture_client_id::FIXTURE_CLIENT_ID;
use margaret_authorization_server_client_tests::fixture_client_secret::fixture_client_secret;
use margaret_authorization_server_client_tests::localhost_discovery_metadata::localhost_discovery_metadata;
use margaret_authorization_server_client_tests::localhost_trust::localhost_trust;
use margaret_authorization_server_client_tests::secret_basic_authentication::secret_basic_authentication;
use margaret_issuer_key_set::issuer_key_set::IssuerKeySet;
use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;
use margaret_oidc_discovery::advertised_endpoint::AdvertisedEndpoint;
use margaret_oidc_discovery::authorization_response_issuer::AuthorizationResponseIssuer;
use margaret_oidc_sign_in::sign_in_beginning::SignInBeginning;
use margaret_oidc_sign_in::sign_in_completion::SignInCompletion;
use margaret_oidc_sign_in::sign_in_flow::SignInFlow;
use margaret_oidc_sign_in::sign_in_refusal::SignInRefusal;
use margaret_oidc_sign_in_tests::begin_sign_in::begin_sign_in;
use margaret_oidc_sign_in_tests::begun_sign_in::BegunSignIn;
use margaret_oidc_sign_in_tests::callback_request::callback_request;
use margaret_oidc_sign_in_tests::email_claims::EmailClaims;
use margaret_oidc_sign_in_tests::sign_in_fixture::SignInFixture;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

#[tokio::test]
async fn refuses_a_response_without_the_advertised_issuer() {
    let fixture = SignInFixture::start(secret_basic_authentication()).await;
    let metadata = Arc::new(IssuerMetadata::awaiting());

    metadata.hold(localhost_discovery_metadata(
        AuthorizationResponseIssuer::Advertised,
        AdvertisedEndpoint::Unadvertised,
    ));

    let advertising = SignInFlow::create(
        Arc::new(AuthorizationServerClient::with_client_secret_basic(
            fixture.server.request_client(),
            Arc::clone(&metadata),
            Arc::new(TrustedIssuer::create(
                Arc::new(IssuerKeySet::awaiting()),
                localhost_trust(),
            )),
            FIXTURE_CLIENT_ID,
            fixture_client_secret(),
        )),
        Arc::clone(&fixture.roller),
    );
    let SignInBeginning::Redirected(response) = begin_sign_in(&advertising).await else {
        panic!("the sign-in redirects to the authorization endpoint");
    };
    let begun = BegunSignIn::of(&response);

    let completion = advertising
        .complete::<EmailClaims>(&callback_request(
            &begun.cookie_pair(),
            &BTreeMap::from([
                ("code", "SplxlOBeZQQYbYS6WxSbIA"),
                ("state", begun.authorization_parameter("state")),
            ]),
        ))
        .await;

    fixture.server.stop().await;

    assert!(matches!(
        completion,
        SignInCompletion::Refused(SignInRefusal::IssuerMissing)
    ));
}
