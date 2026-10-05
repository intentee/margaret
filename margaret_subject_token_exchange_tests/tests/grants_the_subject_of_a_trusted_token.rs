use chrono::Utc;

use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwt_verification::id_token_profile::IdTokenProfile;
use margaret_oauth_vocabulary::subject_token_type::SubjectTokenType;
use margaret_subject_token_exchange::exchanged_subject::ExchangedSubject;
use margaret_subject_token_exchange::subject_token_exchangers::SubjectTokenExchangers;
use margaret_subject_token_exchange_tests::exchanged_subject::EXCHANGED_SUBJECT;
use margaret_subject_token_exchange_tests::trusted_repository_issuer::TrustedRepositoryIssuer;

#[tokio::test]
async fn grants_the_subject_of_a_trusted_token() {
    let ci = TrustedRepositoryIssuer::named("https://ci.localhost");
    let exchangers = SubjectTokenExchangers::create(vec![ci.publishing_keys::<IdTokenProfile>()]);
    let ExchangedSubject::Granted { scopes, subject } = exchangers
        .exchange(
            &ci.token("intentee/margaret", JwtType::Jwt),
            SubjectTokenType::IdToken,
            Utc::now(),
        )
        .await
    else {
        panic!("the exchanger grants the trusted subject");
    };

    assert_eq!(subject, EXCHANGED_SUBJECT);
    assert_eq!(
        scopes
            .iter()
            .map(margaret_oauth_vocabulary::scope::Scope::as_str)
            .collect::<Vec<&str>>(),
        vec!["artifacts:write"]
    );
}
