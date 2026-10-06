use std::collections::BTreeSet;
use std::marker::PhantomData;

use async_trait::async_trait;

use margaret_jwt_verification::verified_jwt::VerifiedJwt;
use margaret_subject_token_exchange::exchanges_subject_tokens::ExchangesSubjectTokens;
use margaret_subject_token_exchange::subject_token_exchange::SubjectTokenExchange;
use margaret_subject_token_exchange::subject_token_profile::SubjectTokenProfile;

use crate::exchanged_subject::EXCHANGED_SUBJECT;
use crate::repository_claims::RepositoryClaims;

pub struct RepositoryExchanger<TProfile> {
    pub profile: PhantomData<TProfile>,
}

#[async_trait]
impl<TProfile: SubjectTokenProfile + Send + Sync + 'static> ExchangesSubjectTokens
    for RepositoryExchanger<TProfile>
{
    type Claims = RepositoryClaims;
    type Profile = TProfile;

    async fn exchange(
        &self,
        token: &VerifiedJwt<RepositoryClaims, TProfile>,
    ) -> anyhow::Result<SubjectTokenExchange> {
        Ok(if token.claims.repository == "intentee/margaret" {
            SubjectTokenExchange::Granted {
                scopes: BTreeSet::from(["artifacts:write"
                    .parse()
                    .expect("the scope is a scope token")]),
                subject: EXCHANGED_SUBJECT,
            }
        } else {
            SubjectTokenExchange::Refused
        })
    }
}
