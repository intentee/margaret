use std::sync::Arc;

use async_trait::async_trait;

use margaret_jwt_verification::attributed_jwt::AttributedJwt;
use margaret_oauth_vocabulary::subject_token_type::SubjectTokenType;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_trusted_issuer::issuer_verification::IssuerVerification;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

use crate::exchanged_subject::ExchangedSubject;
use crate::exchanges_presented_tokens::ExchangesPresentedTokens;
use crate::exchanges_subject_tokens::ExchangesSubjectTokens;
use crate::subject_token_exchange::SubjectTokenExchange;
use crate::subject_token_profile::SubjectTokenProfile;
use crate::subject_token_refusal::SubjectTokenRefusal;

pub(crate) struct TypedExchange<TExchanger> {
    pub(crate) exchanger: Arc<TExchanger>,
}

#[async_trait]
impl<TExchanger: ExchangesSubjectTokens> ExchangesPresentedTokens for TypedExchange<TExchanger> {
    async fn exchange(
        &self,
        trusted_issuer: &TrustedIssuer,
        jwt: &AttributedJwt<'_>,
        token_type: SubjectTokenType,
        now: NumericDate,
    ) -> ExchangedSubject {
        if !TExchanger::Profile::admits(token_type) {
            return ExchangedSubject::Refused(SubjectTokenRefusal::TokenTypeMismatch);
        }

        match trusted_issuer
            .verify::<TExchanger::Claims, TExchanger::Profile>(jwt, now)
            .await
        {
            IssuerVerification::KeysAwaited => ExchangedSubject::SigningKeysAwaited,
            IssuerVerification::Rejected(rejection) => {
                ExchangedSubject::Refused(SubjectTokenRefusal::Rejected(rejection))
            }
            IssuerVerification::Verified(verified) => match self.exchanger.exchange(&verified) {
                SubjectTokenExchange::Granted { scopes, subject } => {
                    ExchangedSubject::Granted { scopes, subject }
                }
                SubjectTokenExchange::Refused => {
                    ExchangedSubject::Refused(SubjectTokenRefusal::ExchangeRefused)
                }
            },
        }
    }
}
