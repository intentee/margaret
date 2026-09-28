use std::sync::Arc;
use std::time::SystemTime;

use margaret_http::request_authorization::RequestAuthorization;
use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_registered_claims::registered_claims_error::RegisteredClaimsError;

use crate::presented_jws::PresentedJws;
use crate::presented_token::PresentedToken;

fn presented_token(authorization: &RequestAuthorization) -> PresentedToken<'_> {
    match authorization {
        RequestAuthorization::Absent => PresentedToken::Absent,
        RequestAuthorization::Bearer(token) => {
            PresentedToken::Bearer(match CompactJws::parse(token.as_str()) {
                CompactJwsParsing::Parsed(jws) => PresentedJws::Parsed(jws),
                CompactJwsParsing::Rejected(rejection) => {
                    PresentedJws::Unparseable(Arc::new(rejection))
                }
            })
        }
        RequestAuthorization::Malformed => PresentedToken::MalformedAuthorization,
        RequestAuthorization::OtherScheme => PresentedToken::NotBearer,
    }
}

pub struct PresentedBearer<'request> {
    pub(crate) now: NumericDate,
    pub(crate) token: PresentedToken<'request>,
}

impl<'request> PresentedBearer<'request> {
    /// # Errors
    ///
    /// Returns `RegisteredClaimsError` when the system clock cannot be read as a numeric date.
    pub fn read(
        authorization: &'request RequestAuthorization,
    ) -> Result<Self, RegisteredClaimsError> {
        NumericDate::from_system_time(SystemTime::now()).map(|now| Self {
            now,
            token: presented_token(authorization),
        })
    }
}
