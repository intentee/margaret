use serde::Deserialize;
use serde_json::Value;

use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

use crate::attributed_jwt::AttributedJwt;
use crate::claims_rejection::ClaimsRejection;
use crate::jwt_attribution::JwtAttribution;
use crate::jwt_presentation::JwtPresentation;
use crate::jwt_rejection::JwtRejection;

#[derive(Deserialize)]
struct IssuerMember {
    iss: String,
}

fn malformed<'token>(source: serde_json::Error) -> JwtPresentation<'token> {
    JwtPresentation::Rejected(JwtRejection::Claims(ClaimsRejection::Malformed { source }))
}

pub struct PresentedJwt<'token> {
    issuer: String,
    jws: CompactJws<'token>,
    payload: Value,
}

impl<'token> PresentedJwt<'token> {
    #[must_use]
    pub fn present(token: &'token str) -> JwtPresentation<'token> {
        let jws = match CompactJws::parse(token) {
            CompactJwsParsing::Parsed(jws) => jws,
            CompactJwsParsing::Rejected(rejection) => {
                return JwtPresentation::Rejected(JwtRejection::Jws(rejection));
            }
        };
        let payload: Value = match serde_json::from_slice(jws.payload()) {
            Ok(payload) => payload,
            Err(source) => return malformed(source),
        };

        match IssuerMember::deserialize(&payload) {
            Ok(IssuerMember { iss }) => JwtPresentation::Presented(Self {
                issuer: iss,
                jws,
                payload,
            }),
            Err(source) => malformed(source),
        }
    }

    #[must_use]
    pub fn attribute_to(self, issuer: &IssuerIdentifier) -> JwtAttribution<'token> {
        if self.issuer == issuer.as_str() {
            JwtAttribution::Attributed(AttributedJwt {
                jws: self.jws,
                payload: self.payload,
            })
        } else {
            JwtAttribution::Unattributed(self)
        }
    }

    #[must_use]
    pub fn issuer(&self) -> &str {
        &self.issuer
    }
}
