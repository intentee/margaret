use chrono::Utc;

use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwt_verification::id_token_profile::IdTokenProfile;
use margaret_oauth_vocabulary::subject_token_type::SubjectTokenType;
use margaret_subject_token_exchange::exchanged_subject::ExchangedSubject;
use margaret_subject_token_exchange::subject_token_exchangers::SubjectTokenExchangers;
use margaret_subject_token_exchange::subject_token_refusal::SubjectTokenRefusal;
use margaret_subject_token_exchange_tests::trusted_repository_issuer::TrustedRepositoryIssuer;

#[tokio::test]
async fn refuses_a_token_of_an_untrusted_issuer() {
    let ci = TrustedRepositoryIssuer::named("https://ci.localhost");
    let stranger = TrustedRepositoryIssuer::named("https://stranger.localhost");
    let exchangers = SubjectTokenExchangers::create(vec![ci.publishing_keys::<IdTokenProfile>()]);

    assert!(matches!(
        exchangers
            .exchange(&stranger.token("intentee/margaret", JwtType::Jwt), SubjectTokenType::IdToken, Utc::now())
            .await,
        ExchangedSubject::Refused(SubjectTokenRefusal::UntrustedIssuer { ref issuer }) if issuer == "https://stranger.localhost"
    ));
}
