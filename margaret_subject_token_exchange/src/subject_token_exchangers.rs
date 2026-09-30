use std::sync::Arc;

use chrono::DateTime;
use chrono::Utc;

use margaret_jwt_verification::jwt_attribution::JwtAttribution;
use margaret_jwt_verification::jwt_presentation::JwtPresentation;
use margaret_jwt_verification::presented_jwt::PresentedJwt;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_token_exchange_client::subject_token_type::SubjectTokenType;

use crate::exchanged_subject::ExchangedSubject;
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

    pub async fn exchange(
        &self,
        subject_token: &str,
        token_type: SubjectTokenType,
        now: DateTime<Utc>,
    ) -> ExchangedSubject {
        let mut presented = match PresentedJwt::present(subject_token) {
            JwtPresentation::Presented(presented) => presented,
            JwtPresentation::Rejected(rejection) => {
                return ExchangedSubject::Refused(SubjectTokenRefusal::Rejected(rejection));
            }
        };

        for exchanger in &self.exchangers {
            match presented.attribute_to(&exchanger.trusted_issuer.trust.token_trust().issuer) {
                JwtAttribution::Attributed(jwt) => {
                    return exchanger
                        .exchange
                        .exchange(
                            &exchanger.trusted_issuer,
                            &jwt,
                            token_type,
                            NumericDate::from(now),
                        )
                        .await;
                }
                JwtAttribution::Unattributed(unattributed) => presented = unattributed,
            }
        }

        ExchangedSubject::Refused(SubjectTokenRefusal::UntrustedIssuer {
            issuer: presented.issuer().to_string(),
        })
    }
}
