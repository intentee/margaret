use std::collections::BTreeMap;
use std::sync::Arc;

use url::Url;

use margaret_authorization_server_client::authorization_server_client::AuthorizationServerClient;
use margaret_authorization_server_client_tests::localhost_trust::localhost_trust;
use margaret_authorization_server_client_tests::secret_basic_client::secret_basic_client;
use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;
use margaret_oidc_discovery::advertised_endpoint::AdvertisedEndpoint;
use margaret_oidc_discovery::authorization_response_issuer::AuthorizationResponseIssuer;
use margaret_oidc_discovery::provider_metadata::ProviderMetadata;
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

fn localhost_url(path: &str) -> Url {
    Url::parse("https://localhost")
        .and_then(|origin| origin.join(path))
        .expect("the localhost endpoint is a url")
}

#[tokio::test]
async fn refuses_a_response_without_the_advertised_issuer() {
    let fixture = SignInFixture::start(secret_basic_client()).await;
    let metadata = Arc::new(IssuerMetadata::awaiting());

    metadata.hold(Arc::new(ProviderMetadata {
        authorization_endpoint: AdvertisedEndpoint::Advertised(localhost_url("/authorize")),
        authorization_response_issuer: AuthorizationResponseIssuer::Advertised,
        introspection_endpoint: AdvertisedEndpoint::Unadvertised,
        jwks_uri: localhost_url("/jwks"),
        token_endpoint: AdvertisedEndpoint::Advertised(localhost_url("/token")),
        userinfo_endpoint: AdvertisedEndpoint::Unadvertised,
    }));

    let advertising = SignInFlow::create(
        Arc::new(AuthorizationServerClient::create(
            fixture.server.request_client(),
            Arc::clone(&metadata),
            Arc::new(TrustedIssuer::for_oidc_issuer(
                metadata,
                Arc::new(localhost_trust()),
            )),
            Arc::new(secret_basic_client()),
        )),
        Arc::clone(&fixture.secret_store),
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
        .await
        .expect("the sign-in completes");

    fixture.server.stop().await;

    assert!(matches!(
        completion,
        SignInCompletion::Refused(SignInRefusal::IssuerMissing)
    ));
}
