use std::collections::BTreeSet;
use std::collections::HashSet;
use std::sync::Mutex;

use async_trait::async_trait;
use chrono::Utc;

use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwt_verification::id_token_profile::IdTokenProfile;
use margaret_jwt_verification::verified_jwt::VerifiedJwt;
use margaret_oauth_vocabulary::subject_token_type::SubjectTokenType;
use margaret_subject_token_exchange::exchanged_subject::ExchangedSubject;
use margaret_subject_token_exchange::exchanges_subject_tokens::ExchangesSubjectTokens;
use margaret_subject_token_exchange::subject_token_exchange::SubjectTokenExchange;
use margaret_subject_token_exchange::subject_token_exchangers::SubjectTokenExchangers;
use margaret_subject_token_exchange::subject_token_refusal::SubjectTokenRefusal;
use margaret_subject_token_exchange_tests::exchanged_subject::EXCHANGED_SUBJECT;
use margaret_subject_token_exchange_tests::repository_claims::RepositoryClaims;
use margaret_subject_token_exchange_tests::trusted_repository_issuer::TrustedRepositoryIssuer;

#[derive(Default)]
struct ReplayRefusingExchanger {
    presented: Mutex<HashSet<String>>,
}

#[async_trait]
impl ExchangesSubjectTokens for ReplayRefusingExchanger {
    type Claims = RepositoryClaims;
    type Profile = IdTokenProfile;

    async fn exchange(
        &self,
        token: &VerifiedJwt<RepositoryClaims, IdTokenProfile>,
    ) -> anyhow::Result<SubjectTokenExchange> {
        let identifier = token
            .registered
            .jti
            .clone()
            .expect("the subject token carries an identifier");
        let first_presentation = self
            .presented
            .lock()
            .expect("the presented identifiers are readable")
            .insert(identifier);

        Ok(if first_presentation {
            SubjectTokenExchange::Granted {
                scopes: BTreeSet::new(),
                subject: EXCHANGED_SUBJECT,
            }
        } else {
            SubjectTokenExchange::Refused
        })
    }
}

#[tokio::test]
async fn hands_the_token_identifier_to_the_exchanger() {
    let ci = TrustedRepositoryIssuer::named("https://ci.localhost");
    let exchangers = SubjectTokenExchangers::create(vec![
        ci.exchanging_with(ReplayRefusingExchanger::default()),
    ]);
    let token = ci.token("intentee/margaret", JwtType::Jwt);
    let first = exchangers
        .exchange(&token, SubjectTokenType::IdToken, Utc::now())
        .await
        .expect("the exchangers complete the first exchange");
    let replayed = exchangers
        .exchange(&token, SubjectTokenType::IdToken, Utc::now())
        .await
        .expect("the exchangers complete the replayed exchange");

    assert!(matches!(first, ExchangedSubject::Granted { .. }));
    assert!(matches!(
        replayed,
        ExchangedSubject::Refused(SubjectTokenRefusal::ExchangeRefused)
    ));
}
