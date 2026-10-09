use async_trait::async_trait;

use margaret_jwt_verification::id_token_profile::IdTokenProfile;
use margaret_jwt_verification::verified_jwt::VerifiedJwt;
use margaret_subject_token_exchange::exchanges_subject_tokens::ExchangesSubjectTokens;
use margaret_subject_token_exchange::subject_token_exchange::SubjectTokenExchange;

use crate::ci_claims::CiClaims;
use crate::ci_subject::CI_SUBJECT;
use crate::fixture_scopes::fixture_scopes;

pub struct CiExchanger;

#[async_trait]
impl ExchangesSubjectTokens for CiExchanger {
    type Claims = CiClaims;
    type Profile = IdTokenProfile;

    async fn exchange(
        &self,
        token: &VerifiedJwt<CiClaims, IdTokenProfile>,
    ) -> anyhow::Result<SubjectTokenExchange> {
        Ok(if token.claims.repository == "intentee/margaret" {
            SubjectTokenExchange::Granted {
                scopes: fixture_scopes(&["profile"]),
                subject: CI_SUBJECT,
            }
        } else {
            SubjectTokenExchange::Refused
        })
    }
}
