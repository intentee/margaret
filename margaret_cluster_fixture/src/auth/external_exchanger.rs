use std::collections::BTreeSet;

use async_trait::async_trait;

use margaret::framework::jwt_verification::id_token_profile::IdTokenProfile;
use margaret::framework::jwt_verification::verified_jwt::VerifiedJwt;
use margaret::framework::macros::exchanges_tokens_from;
use margaret::framework::macros::singleton;
use margaret::framework::subject_token_exchange::exchanges_subject_tokens::ExchangesSubjectTokens;
use margaret::framework::subject_token_exchange::subject_token_exchange::SubjectTokenExchange;

use crate::auth::external_claims::ExternalClaims;
use crate::stores::alice::ALICE;

#[singleton]
#[exchanges_tokens_from(issuer = external)]
pub struct ExternalExchanger;

#[async_trait]
impl ExchangesSubjectTokens for ExternalExchanger {
    type Claims = ExternalClaims;
    type Profile = IdTokenProfile;

    async fn exchange(
        &self,
        token: &VerifiedJwt<ExternalClaims, IdTokenProfile>,
    ) -> anyhow::Result<SubjectTokenExchange> {
        Ok(if token.claims.sub == "alice" {
            SubjectTokenExchange::Granted {
                scopes: BTreeSet::new(),
                subject: ALICE,
            }
        } else {
            SubjectTokenExchange::Refused
        })
    }
}
