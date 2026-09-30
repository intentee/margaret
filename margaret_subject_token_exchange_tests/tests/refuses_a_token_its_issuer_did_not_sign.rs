use chrono::Utc;

use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwt_verification::id_token_profile::IdTokenProfile;
use margaret_subject_token_exchange::exchanged_subject::ExchangedSubject;
use margaret_subject_token_exchange::subject_token_exchangers::SubjectTokenExchangers;
use margaret_subject_token_exchange::subject_token_refusal::SubjectTokenRefusal;
use margaret_subject_token_exchange_tests::signed_by::signed_by;
use margaret_subject_token_exchange_tests::trusted_repository_issuer::TrustedRepositoryIssuer;
use margaret_token_exchange_client::subject_token_type::SubjectTokenType;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;

#[tokio::test]
async fn refuses_a_token_its_issuer_did_not_sign() {
    let ci = TrustedRepositoryIssuer::named("https://ci.localhost");
    let exchangers = SubjectTokenExchangers::create(vec![ci.publishing_keys::<IdTokenProfile>()]);
    let forged = signed_by(
        &fresh_p256_secret(),
        "https://ci.localhost",
        "intentee/margaret",
        JwtType::Jwt,
    );

    assert!(matches!(
        exchangers
            .exchange(&forged, SubjectTokenType::IdToken, Utc::now())
            .await,
        ExchangedSubject::Refused(SubjectTokenRefusal::Rejected(_))
    ));
}
