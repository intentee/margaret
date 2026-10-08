use std::collections::BTreeMap;
use std::sync::Arc;

use margaret_authorization_server_client::authorization_server_client::AuthorizationServerClient;
use margaret_authorization_server_client_tests::fixture_client_secret::fixture_client_secret;
use margaret_authorization_server_client_tests::localhost_discovery_metadata::localhost_discovery_metadata;
use margaret_authorization_server_client_tests::localhost_trust::localhost_trust;
use margaret_authorization_server_client_tests::secret_basic_authentication::secret_basic_authentication;
use margaret_issuer_key_set::issuer_key_set::IssuerKeySet;
use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;
use margaret_jwt_verification::claims_rejection::ClaimsRejection;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_oidc_discovery::advertised_endpoint::AdvertisedEndpoint;
use margaret_oidc_discovery::authorization_response_issuer::AuthorizationResponseIssuer;
use margaret_oidc_sign_in::sign_in_beginning::SignInBeginning;
use margaret_oidc_sign_in::sign_in_completion::SignInCompletion;
use margaret_oidc_sign_in::sign_in_refusal::SignInRefusal;
use margaret_oidc_sign_in_tests::begun_sign_in::BegunSignIn;
use margaret_oidc_sign_in_tests::callback_request::callback_request;
use margaret_oidc_sign_in_tests::email_claims::EmailClaims;
use margaret_oidc_sign_in_tests::fixture_sign_in_flow::fixture_sign_in_flow;
use margaret_oidc_sign_in_tests::sign_in_fixture::SignInFixture;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

#[tokio::test]
async fn refuses_a_transaction_sealed_for_another_client() {
    let fixture = SignInFixture::start(secret_basic_authentication()).await;
    let metadata = Arc::new(IssuerMetadata::awaiting());

    metadata.hold(localhost_discovery_metadata(
        AuthorizationResponseIssuer::Unadvertised,
        AdvertisedEndpoint::Unadvertised,
    ));

    let another_client = fixture_sign_in_flow(
        Arc::new(AuthorizationServerClient::with_client_secret_basic(
            fixture.server.request_client(),
            metadata,
            Arc::new(TrustedIssuer::polled(
                Arc::new(IssuerKeySet::awaiting()),
                localhost_trust(),
            )),
            "another:client",
            fixture_client_secret(),
        )),
        Arc::clone(&fixture.roller),
    );
    let SignInBeginning::Redirected(sealed) = fixture.flow.begin().await else {
        panic!("the sign-in redirects to the authorization endpoint");
    };
    let SignInBeginning::Redirected(expected) = another_client.begin().await else {
        panic!("the sign-in of another client redirects to the authorization endpoint");
    };
    let sealed = BegunSignIn::of(&sealed);
    let expected = BegunSignIn::of(&expected);

    let completion = another_client
        .complete::<EmailClaims>(&callback_request(
            &format!("{}={}", expected.cookie.name(), sealed.cookie.value()),
            &BTreeMap::from([
                ("code", "SplxlOBeZQQYbYS6WxSbIA"),
                ("state", sealed.authorization_parameter("state")),
            ]),
        ))
        .await;

    fixture.server.stop().await;

    assert!(matches!(
        completion,
        SignInCompletion::Refused(SignInRefusal::TransactionRejected(JwtRejection::Claims(
            ClaimsRejection::AudienceMismatch { .. }
        )))
    ));
}
