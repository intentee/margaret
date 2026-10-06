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
use crate::subject_token_exchange_error::SubjectTokenExchangeError;
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
    ) -> Result<ExchangedSubject, SubjectTokenExchangeError> {
        if !TExchanger::Profile::admits(token_type) {
            return Ok(ExchangedSubject::Refused(
                SubjectTokenRefusal::TokenTypeMismatch,
            ));
        }

        let verified = match trusted_issuer
            .verify::<TExchanger::Claims, TExchanger::Profile>(jwt, now)
            .await
        {
            IssuerVerification::KeysAwaited => return Ok(ExchangedSubject::SigningKeysAwaited),
            IssuerVerification::Rejected(rejection) => {
                return Ok(ExchangedSubject::Refused(SubjectTokenRefusal::Rejected(
                    rejection,
                )));
            }
            IssuerVerification::Verified(verified) => verified,
        };

        match self.exchanger.exchange(&verified).await {
            Ok(SubjectTokenExchange::Granted { scopes, subject }) => {
                Ok(ExchangedSubject::Granted { scopes, subject })
            }
            Ok(SubjectTokenExchange::Refused) => Ok(ExchangedSubject::Refused(
                SubjectTokenRefusal::ExchangeRefused,
            )),
            Err(source) => Err(SubjectTokenExchangeError::ExchangerFailed { source }),
        }
    }
}
