use std::ops::ControlFlow;

use crate::attributed_jwt::AttributedJwt;
use crate::jwt_expectation::JwtExpectation;
use crate::jwt_presentation::JwtPresentation;
use crate::jwt_rejection::JwtRejection;
use crate::presented_jwt::PresentedJwt;

pub fn attribute_serialized_jwt<'token>(
    token: &'token str,
    expectation: &JwtExpectation,
) -> ControlFlow<JwtRejection, AttributedJwt<'token>> {
    match PresentedJwt::present(token) {
        JwtPresentation::Presented(presented) => presented.attribute_to(expectation),
        JwtPresentation::Rejected(rejection) => ControlFlow::Break(rejection),
    }
}
