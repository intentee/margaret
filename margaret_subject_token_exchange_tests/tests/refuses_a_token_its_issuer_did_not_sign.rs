use chrono::Utc;
use serde_json::json;

use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwt_verification::id_token_profile::IdTokenProfile;
use margaret_oauth_vocabulary::subject_token_type::SubjectTokenType;
use margaret_subject_token_exchange::exchanged_subject::ExchangedSubject;
use margaret_subject_token_exchange::subject_token_exchangers::SubjectTokenExchangers;
use margaret_subject_token_exchange::subject_token_refusal::SubjectTokenRefusal;
use margaret_subject_token_exchange_tests::provider_audience::PROVIDER_AUDIENCE;
use margaret_subject_token_exchange_tests::signed_by::signed_by;
use margaret_subject_token_exchange_tests::trusted_repository_issuer::TrustedRepositoryIssuer;

#[tokio::test]
async fn refuses_a_token_its_issuer_did_not_sign() {
    let ci = TrustedRepositoryIssuer::named("https://ci.localhost");
    let exchangers = SubjectTokenExchangers::create(vec![ci.publishing_keys::<IdTokenProfile>()]);
    let forged = signed_by(
        &fresh_secret(SigningCurve::P256),
        "https://ci.localhost",
        &json!(PROVIDER_AUDIENCE),
        "intentee/margaret",
        JwtType::Jwt,
    );

    assert!(matches!(
        exchangers
            .exchange(&forged, SubjectTokenType::IdToken, Utc::now())
            .await
            .expect("the exchangers complete the exchange"),
        ExchangedSubject::Refused(SubjectTokenRefusal::Rejected(_))
    ));
}
