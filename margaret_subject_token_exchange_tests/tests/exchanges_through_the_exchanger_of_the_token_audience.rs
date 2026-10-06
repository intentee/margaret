use chrono::Utc;

use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwt_verification::id_token_profile::IdTokenProfile;
use margaret_oauth_vocabulary::subject_token_type::SubjectTokenType;
use margaret_subject_token_exchange::exchanged_subject::ExchangedSubject;
use margaret_subject_token_exchange::subject_token_exchangers::SubjectTokenExchangers;
use margaret_subject_token_exchange_tests::exchanged_subject::EXCHANGED_SUBJECT;
use margaret_subject_token_exchange_tests::trusted_repository_issuer::TrustedRepositoryIssuer;

#[tokio::test]
async fn exchanges_through_the_exchanger_of_the_token_audience() {
    let ci = TrustedRepositoryIssuer::named("https://ci.localhost");
    let exchangers = SubjectTokenExchangers::create(vec![
        ci.publishing_keys_for::<IdTokenProfile>("https://elsewhere.localhost"),
        ci.publishing_keys::<IdTokenProfile>(),
    ]);

    assert!(matches!(
        exchangers
            .exchange(
                &ci.token("intentee/margaret", JwtType::Jwt),
                SubjectTokenType::IdToken,
                Utc::now(),
            )
            .await,
        ExchangedSubject::Granted { subject, .. } if subject == EXCHANGED_SUBJECT
    ));
}
