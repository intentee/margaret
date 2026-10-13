use anyhow::anyhow;
use async_trait::async_trait;
use chrono::Utc;

use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwt_verification::id_token_profile::IdTokenProfile;
use margaret_jwt_verification::verified_jwt::VerifiedJwt;
use margaret_oauth_vocabulary::subject_token_type::SubjectTokenType;
use margaret_subject_token_exchange::exchanges_subject_tokens::ExchangesSubjectTokens;
use margaret_subject_token_exchange::subject_token_exchange::SubjectTokenExchange;
use margaret_subject_token_exchange::subject_token_exchange_error::SubjectTokenExchangeError;
use margaret_subject_token_exchange::subject_token_exchangers::SubjectTokenExchangers;
use margaret_subject_token_exchange_tests::repository_claims::RepositoryClaims;
use margaret_subject_token_exchange_tests::trusted_repository_issuer::TrustedRepositoryIssuer;

struct UnreachableStoreExchanger;

#[async_trait]
impl ExchangesSubjectTokens for UnreachableStoreExchanger {
    type Claims = RepositoryClaims;
    type Profile = IdTokenProfile;

    async fn exchange(
        &self,
        _token: &VerifiedJwt<RepositoryClaims, IdTokenProfile>,
    ) -> anyhow::Result<SubjectTokenExchange> {
        Err(anyhow!("the presented token store is unreachable"))
    }
}

#[tokio::test]
async fn reports_a_failing_exchanger_as_a_system_error() {
    let ci = TrustedRepositoryIssuer::named("https://ci.localhost");
    let exchangers =
        SubjectTokenExchangers::create(vec![ci.exchanging_with(UnreachableStoreExchanger)]);

    assert!(matches!(
        exchangers
            .exchange(
                &ci.token("intentee/margaret", JwtType::Jwt),
                SubjectTokenType::IdToken,
                Utc::now(),
            )
            .await,
        Err(SubjectTokenExchangeError::ExchangerFailed { source })
            if source.to_string() == "the presented token store is unreachable"
    ));
}
