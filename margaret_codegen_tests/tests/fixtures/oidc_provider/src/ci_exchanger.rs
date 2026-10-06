use std::collections::BTreeSet;

use async_trait::async_trait;
use uuid::Uuid;

use margaret::framework::jwt_verification::id_token_profile::IdTokenProfile;
use margaret::framework::jwt_verification::verified_jwt::VerifiedJwt;
use margaret::framework::macros::exchanges_subject_tokens;
use margaret::framework::macros::singleton;
use margaret::framework::subject_token_exchange::exchanges_subject_tokens::ExchangesSubjectTokens;
use margaret::framework::subject_token_exchange::subject_token_exchange::SubjectTokenExchange;

use crate::ci_claims::CiClaims;

#[singleton]
#[exchanges_subject_tokens(issuer = ci)]
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
                scopes: BTreeSet::new(),
                subject: Uuid::nil(),
            }
        } else {
            SubjectTokenExchange::Refused
        })
    }
}
