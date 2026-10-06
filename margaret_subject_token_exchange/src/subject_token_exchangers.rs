use std::sync::Arc;

use chrono::DateTime;
use chrono::Utc;

use margaret_jwt_verification::jwt_presentation::JwtPresentation;
use margaret_jwt_verification::jwt_routing::JwtRouting;
use margaret_jwt_verification::presented_jwt::PresentedJwt;
use margaret_oauth_vocabulary::subject_token_type::SubjectTokenType;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_registered_claims::registered_claims::RegisteredClaims;

use crate::exchanged_subject::ExchangedSubject;
use crate::subject_token_exchange_error::SubjectTokenExchangeError;
use crate::subject_token_exchanger::SubjectTokenExchanger;
use crate::subject_token_refusal::SubjectTokenRefusal;

pub struct SubjectTokenExchangers {
    exchangers: Vec<Arc<SubjectTokenExchanger>>,
}

impl SubjectTokenExchangers {
    #[must_use]
    pub fn create(exchangers: Vec<Arc<SubjectTokenExchanger>>) -> Self {
        Self { exchangers }
    }

    /// # Errors
    ///
    /// Returns `SubjectTokenExchangeError::ExchangerFailed` when the exchanger the token is
    /// addressed to fails.
    pub async fn exchange(
        &self,
        subject_token: &str,
        token_type: SubjectTokenType,
        now: DateTime<Utc>,
    ) -> Result<ExchangedSubject, SubjectTokenExchangeError> {
        let presented = match PresentedJwt::present(subject_token) {
            JwtPresentation::Presented(presented) => presented,
            JwtPresentation::Rejected(rejection) => {
                return Ok(ExchangedSubject::Refused(SubjectTokenRefusal::Rejected(
                    rejection,
                )));
            }
        };

        match presented.route(self.exchangers.iter().map(Arc::as_ref)) {
            JwtRouting::Routed {
                addressee: exchanger,
                jwt,
            } => {
                exchanger
                    .exchange
                    .exchange(
                        &exchanger.trusted_issuer,
                        &jwt,
                        token_type,
                        NumericDate::from(now),
                    )
                    .await
            }
            JwtRouting::Ambiguous(RegisteredClaims { aud, iss, .. }) => {
                Ok(ExchangedSubject::Refused(SubjectTokenRefusal::Ambiguous {
                    audience: aud,
                    issuer: iss,
                }))
            }
            JwtRouting::Misaddressed(RegisteredClaims { aud, iss, .. }) => Ok(
                ExchangedSubject::Refused(SubjectTokenRefusal::Misaddressed {
                    audience: aud,
                    issuer: iss,
                }),
            ),
            JwtRouting::UntrustedIssuer(RegisteredClaims { iss, .. }) => Ok(
                ExchangedSubject::Refused(SubjectTokenRefusal::UntrustedIssuer { issuer: iss }),
            ),
        }
    }
}
