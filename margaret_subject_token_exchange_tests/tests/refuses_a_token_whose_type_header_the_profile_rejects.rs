use chrono::Utc;

use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwt_verification::access_token_profile::AccessTokenProfile;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_oauth_vocabulary::subject_token_type::SubjectTokenType;
use margaret_subject_token_exchange::exchanged_subject::ExchangedSubject;
use margaret_subject_token_exchange::subject_token_exchangers::SubjectTokenExchangers;
use margaret_subject_token_exchange::subject_token_refusal::SubjectTokenRefusal;
use margaret_subject_token_exchange_tests::trusted_repository_issuer::TrustedRepositoryIssuer;

#[tokio::test]
async fn refuses_a_token_whose_type_header_the_profile_rejects() {
    let ci = TrustedRepositoryIssuer::named("https://ci.localhost");
    let exchangers =
        SubjectTokenExchangers::create(vec![ci.publishing_keys::<AccessTokenProfile>()]);

    assert!(matches!(
        exchangers
            .exchange(
                &ci.token("intentee/margaret", JwtType::Jwt),
                SubjectTokenType::AccessToken,
                Utc::now()
            )
            .await
            .expect("the exchangers complete the exchange"),
        ExchangedSubject::Refused(SubjectTokenRefusal::Rejected(JwtRejection::Type(_)))
    ));
}
