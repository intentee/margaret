use chrono::Utc;

use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwt_verification::id_token_profile::IdTokenProfile;
use margaret_subject_token_exchange::exchanged_subject::ExchangedSubject;
use margaret_subject_token_exchange::subject_token_exchangers::SubjectTokenExchangers;
use margaret_subject_token_exchange::subject_token_refusal::SubjectTokenRefusal;
use margaret_subject_token_exchange_tests::trusted_repository_issuer::TrustedRepositoryIssuer;
use margaret_token_exchange_client::subject_token_type::SubjectTokenType;

#[tokio::test]
async fn refuses_a_token_its_exchanger_refuses() {
    let ci = TrustedRepositoryIssuer::named("https://ci.localhost");
    let exchangers = SubjectTokenExchangers::create(vec![ci.publishing_keys::<IdTokenProfile>()]);

    assert!(matches!(
        exchangers
            .exchange(
                &ci.token("someone/else", JwtType::Jwt),
                SubjectTokenType::Jwt,
                Utc::now()
            )
            .await,
        ExchangedSubject::Refused(SubjectTokenRefusal::ExchangeRefused)
    ));
}
