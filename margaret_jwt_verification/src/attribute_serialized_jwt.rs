use std::ops::ControlFlow;

use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

use crate::attributed_jwt::AttributedJwt;
use crate::claims_rejection::ClaimsRejection;
use crate::jwt_attribution::JwtAttribution;
use crate::jwt_presentation::JwtPresentation;
use crate::jwt_rejection::JwtRejection;
use crate::presented_jwt::PresentedJwt;

pub fn attribute_serialized_jwt<'token>(
    token: &'token str,
    issuer: &IssuerIdentifier,
) -> ControlFlow<JwtRejection, AttributedJwt<'token>> {
    match PresentedJwt::present(token) {
        JwtPresentation::Presented(presented) => match presented.attribute_to(issuer) {
            JwtAttribution::Attributed(attributed) => ControlFlow::Continue(attributed),
            JwtAttribution::Unattributed(unattributed) => {
                ControlFlow::Break(JwtRejection::Claims(ClaimsRejection::IssuerMismatch {
                    expected: issuer.clone(),
                    found: unattributed.issuer().to_string(),
                }))
            }
        },
        JwtPresentation::Rejected(rejection) => ControlFlow::Break(rejection),
    }
}
