use chrono::Utc;
use serde_json::json;

use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwt_verification::id_token_profile::IdTokenProfile;
use margaret_oauth_vocabulary::subject_token_type::SubjectTokenType;
use margaret_subject_token_exchange::exchanged_subject::ExchangedSubject;
use margaret_subject_token_exchange::subject_token_exchangers::SubjectTokenExchangers;
use margaret_subject_token_exchange::subject_token_refusal::SubjectTokenRefusal;
use margaret_subject_token_exchange_tests::provider_audience::PROVIDER_AUDIENCE;
use margaret_subject_token_exchange_tests::trusted_repository_issuer::TrustedRepositoryIssuer;

#[tokio::test]
async fn refuses_a_subject_token_addressed_to_two_exchangers() {
    let ci = TrustedRepositoryIssuer::named("https://ci.localhost");
    let exchangers = SubjectTokenExchangers::create(vec![
        ci.publishing_keys::<IdTokenProfile>(),
        ci.publishing_keys_for::<IdTokenProfile>("https://elsewhere.localhost"),
    ]);

    assert!(matches!(
        exchangers
            .exchange(
                &ci.token_for(
                    &json!([PROVIDER_AUDIENCE, "https://elsewhere.localhost"]),
                    "intentee/margaret",
                    JwtType::Jwt,
                ),
                SubjectTokenType::IdToken,
                Utc::now(),
            )
            .await
        .expect("the exchangers complete the exchange"),
        ExchangedSubject::Refused(SubjectTokenRefusal::Ambiguous { ref issuer, .. })
            if issuer == "https://ci.localhost"
    ));
}
