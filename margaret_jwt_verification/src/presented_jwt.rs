use std::ops::ControlFlow;

use serde::Deserialize;
use serde_json::Map;
use serde_json::Value;

use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_registered_claims::registered_claims::RegisteredClaims;

use crate::attributed_jwt::AttributedJwt;
use crate::claims_rejection::ClaimsRejection;
use crate::jwt_addressee::JwtAddressee;
use crate::jwt_expectation::JwtExpectation;
use crate::jwt_presentation::JwtPresentation;
use crate::jwt_rejection::JwtRejection;
use crate::jwt_routing::JwtRouting;

#[derive(Deserialize)]
struct PresentedClaims {
    #[serde(flatten)]
    registered: RegisteredClaims,
    #[serde(flatten)]
    application: Map<String, Value>,
}

pub struct PresentedJwt<'token> {
    application: Value,
    jws: CompactJws<'token>,
    registered: RegisteredClaims,
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

        match serde_json::from_slice::<PresentedClaims>(jws.payload()) {
            Ok(PresentedClaims {
                registered,
                application,
            }) => JwtPresentation::Presented(Self {
                application: Value::Object(application),
                jws,
                registered,
            }),
            Err(source) => {
                JwtPresentation::Rejected(JwtRejection::Claims(ClaimsRejection::Malformed {
                    source,
                }))
            }
        }
    }

    pub fn attribute_to(
        self,
        JwtExpectation { audience, issuer }: &JwtExpectation,
    ) -> ControlFlow<JwtRejection, AttributedJwt<'token>> {
        if self.registered.iss != issuer.as_str() {
            return ControlFlow::Break(JwtRejection::Claims(ClaimsRejection::IssuerMismatch {
                expected: (*issuer).clone(),
                found: self.registered.iss,
            }));
        }

        if let ControlFlow::Break(rejection) = audience.check(&self.registered.aud) {
            return ControlFlow::Break(JwtRejection::Claims(rejection));
        }

        ControlFlow::Continue(self.attributed())
    }

    #[must_use]
    pub fn route<'addressees, TAddressee: JwtAddressee>(
        self,
        addressees: impl IntoIterator<Item = &'addressees TAddressee>,
    ) -> JwtRouting<'token, 'addressees, TAddressee> {
        let mut issuer_trusted = false;
        let mut admitting = Vec::new();

        for addressee in addressees {
            let JwtExpectation { audience, issuer } = addressee.jwt_expectation();

            if self.registered.iss == issuer.as_str() {
                issuer_trusted = true;

                if audience.admits(&self.registered.aud) {
                    admitting.push(addressee);
                }
            }
        }

        match admitting.as_slice() {
            [addressee] => JwtRouting::Routed {
                addressee: *addressee,
                jwt: self.attributed(),
            },
            [] if issuer_trusted => JwtRouting::Misaddressed(self.registered),
            [] => JwtRouting::UntrustedIssuer(self.registered),
            [_, _, ..] => JwtRouting::Ambiguous(self.registered),
        }
    }

    fn attributed(self) -> AttributedJwt<'token> {
        AttributedJwt {
            application: self.application,
            jws: self.jws,
            registered: self.registered,
        }
    }
}
