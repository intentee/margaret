use chrono::Utc;

use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwt_verification::id_token_profile::IdTokenProfile;
use margaret_oauth_vocabulary::subject_token_type::SubjectTokenType;
use margaret_subject_token_exchange::exchanged_subject::ExchangedSubject;
use margaret_subject_token_exchange::subject_token_exchangers::SubjectTokenExchangers;
use margaret_subject_token_exchange::subject_token_refusal::SubjectTokenRefusal;
use margaret_subject_token_exchange_tests::trusted_repository_issuer::TrustedRepositoryIssuer;

#[tokio::test]
async fn refuses_a_token_type_the_profile_of_its_exchanger_does_not_admit() {
    let ci = TrustedRepositoryIssuer::named("https://ci.localhost");
    let exchangers = SubjectTokenExchangers::create(vec![ci.publishing_keys::<IdTokenProfile>()]);

    assert!(matches!(
        exchangers
            .exchange(
                &ci.token("intentee/margaret", JwtType::Jwt),
                SubjectTokenType::AccessToken,
                Utc::now()
            )
            .await
            .expect("the exchangers complete the exchange"),
        ExchangedSubject::Refused(SubjectTokenRefusal::TokenTypeMismatch)
    ));
}
