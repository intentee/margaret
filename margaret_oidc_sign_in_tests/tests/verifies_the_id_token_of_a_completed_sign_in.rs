use std::collections::BTreeMap;

use serde_json::Value;
use serde_json::json;

use margaret_authorization_server_client_tests::secret_basic_authentication::secret_basic_authentication;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwt_verification::claims_rejection::ClaimsRejection;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_oidc_sign_in::sign_in_beginning::SignInBeginning;
use margaret_oidc_sign_in::sign_in_completion::SignInCompletion;
use margaret_oidc_sign_in::sign_in_refusal::SignInRefusal;
use margaret_oidc_sign_in::signed_in::SignedIn;
use margaret_oidc_sign_in_tests::begun_sign_in::BegunSignIn;
use margaret_oidc_sign_in_tests::callback_request::callback_request;
use margaret_oidc_sign_in_tests::email_claims::EmailClaims;
use margaret_oidc_sign_in_tests::id_token_claims::id_token_claims;
use margaret_oidc_sign_in_tests::issued_token_answer::issued_token_answer;
use margaret_oidc_sign_in_tests::sign_in_fixture::SignInFixture;

async fn completed_with(
    adjust: impl FnOnce(&mut Value),
    jwt_type: JwtType,
) -> SignInCompletion<EmailClaims> {
    let fixture = SignInFixture::start(secret_basic_authentication()).await;
    let SignInBeginning::Redirected(response) = fixture.flow.begin().await else {
        panic!("the sign-in redirects to the authorization endpoint");
    };
    let begun = BegunSignIn::of(&response);
    let mut claims = id_token_claims(begun.authorization_parameter("nonce"));

    adjust(&mut claims);

    assert!(
        fixture
            .token_endpoint
            .answer
            .set(issued_token_answer(
                &fixture.issuer_secret.current().sign_json(&claims, jwt_type)
            ))
            .is_ok()
    );

    let completion = fixture
        .flow
        .complete::<EmailClaims>(&callback_request(
            &begun.cookie_pair(),
            &BTreeMap::from([
                ("code", "SplxlOBeZQQYbYS6WxSbIA"),
                ("iss", "https://localhost"),
                ("state", begun.authorization_parameter("state")),
            ]),
        ))
        .await;

    fixture.server.stop().await;

    completion
}

#[tokio::test]
async fn accepts_an_id_token_listing_only_this_client() {
    assert!(matches!(
        completed_with(|claims| claims["aud"] = json!(["client:id"]), JwtType::Jwt).await,
        SignInCompletion::SignedIn(_)
    ));
}

#[tokio::test]
async fn refuses_an_access_token_as_an_id_token() {
    assert!(matches!(
        completed_with(|_claims| {}, JwtType::AccessToken).await,
        SignInCompletion::Refused(SignInRefusal::IdTokenRejected(JwtRejection::Type(_)))
    ));
}

#[tokio::test]
async fn refuses_an_id_token_for_another_audience() {
    assert!(matches!(
        completed_with(|claims| claims["aud"] = json!("other"), JwtType::Jwt).await,
        SignInCompletion::Refused(SignInRefusal::IdTokenRejected(JwtRejection::Claims(
            ClaimsRejection::AudienceMismatch { .. }
        )))
    ));
}

#[tokio::test]
async fn refuses_an_id_token_issued_to_another_party() {
    assert!(matches!(
        completed_with(|claims| claims["azp"] = json!("other"), JwtType::Jwt).await,
        SignInCompletion::Refused(SignInRefusal::AuthorizedPartyMismatch { found }) if found == "other"
    ));
}

#[tokio::test]
async fn refuses_an_id_token_of_another_issuer() {
    assert!(matches!(
        completed_with(|claims| claims["iss"] = json!("https://attacker.example"), JwtType::Jwt).await,
        SignInCompletion::Refused(SignInRefusal::IdTokenRejected(JwtRejection::Claims(ClaimsRejection::IssuerMismatch { found, .. }))) if found == "https://attacker.example"
    ));
}

#[tokio::test]
async fn refuses_an_id_token_shared_with_other_audiences() {
    assert!(matches!(
        completed_with(
            |claims| claims["aud"] = json!(["client:id", "other"]),
            JwtType::Jwt
        )
        .await,
        SignInCompletion::Refused(SignInRefusal::AudienceNotExclusive { .. })
    ));
}

#[tokio::test]
async fn refuses_an_id_token_with_another_nonce() {
    assert!(matches!(
        completed_with(|claims| claims["nonce"] = json!("replayed"), JwtType::Jwt).await,
        SignInCompletion::Refused(SignInRefusal::NonceMismatch)
    ));
}

#[tokio::test]
async fn signs_in_with_a_verified_id_token() {
    let SignInCompletion::SignedIn(SignedIn {
        access_token,
        claims,
        subject,
        transaction_removal,
    }) = completed_with(|_claims| {}, JwtType::Jwt).await
    else {
        panic!("the user signs in");
    };

    assert_eq!(access_token.secret(), "2YotnFZFEjr1zCsicMWpAA");
    assert_eq!(claims.email, "user@example.test");
    assert_eq!(subject, "subject");
    assert!(
        transaction_removal
            .name()
            .starts_with("__Host-margaret-sign-in-")
    );
    assert_eq!(
        transaction_removal.max_age(),
        Some(cookie::time::Duration::ZERO)
    );
}
