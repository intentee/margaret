use chrono::Utc;
use serde_json::json;

use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwt_verification::id_token_profile::IdTokenProfile;
use margaret_oauth_vocabulary::subject_token_type::SubjectTokenType;
use margaret_subject_token_exchange::exchanged_subject::ExchangedSubject;
use margaret_subject_token_exchange::subject_token_exchangers::SubjectTokenExchangers;
use margaret_subject_token_exchange::subject_token_refusal::SubjectTokenRefusal;
use margaret_subject_token_exchange_tests::trusted_repository_issuer::TrustedRepositoryIssuer;

#[tokio::test]
async fn refuses_a_subject_token_for_an_audience_its_issuer_is_not_trusted_for() {
    let ci = TrustedRepositoryIssuer::named("https://ci.localhost");
    let exchangers = SubjectTokenExchangers::create(vec![ci.publishing_keys::<IdTokenProfile>()]);

    assert!(matches!(
        exchangers
            .exchange(
                &ci.token_for(&json!("https://elsewhere.localhost"), "intentee/margaret", JwtType::Jwt),
                SubjectTokenType::IdToken,
                Utc::now(),
            )
            .await,
        ExchangedSubject::Refused(SubjectTokenRefusal::Misaddressed { ref audience, ref issuer })
            if audience.to_string() == "https://elsewhere.localhost" && issuer == "https://ci.localhost"
    ));
}
