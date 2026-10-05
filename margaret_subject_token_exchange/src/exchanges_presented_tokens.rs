use async_trait::async_trait;

use margaret_jwt_verification::attributed_jwt::AttributedJwt;
use margaret_oauth_vocabulary::subject_token_type::SubjectTokenType;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

use crate::exchanged_subject::ExchangedSubject;

#[async_trait]
pub(crate) trait ExchangesPresentedTokens: Send + Sync {
    async fn exchange(
        &self,
        trusted_issuer: &TrustedIssuer,
        jwt: &AttributedJwt<'_>,
        token_type: SubjectTokenType,
        now: NumericDate,
    ) -> ExchangedSubject;
}
