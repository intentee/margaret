use std::ops::ControlFlow;
use std::time::SystemTime;

use margaret_http::bearer_challenge::BearerChallenge;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_jwt_verification::jwt_presentation::JwtPresentation;
use margaret_jwt_verification::jwt_routing::JwtRouting;
use margaret_jwt_verification::presented_jwt::PresentedJwt;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_registered_claims::registered_claims_error::RegisteredClaimsError;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

use crate::attributed_bearer_token::AttributedBearerToken;
use crate::bearer_token_routing::BearerTokenRouting;
use crate::routed_bearer_token::RoutedBearerToken;

fn refused<'request, 'trusted>(
    challenge: BearerChallenge,
) -> BearerTokenRouting<'request, 'trusted> {
    BearerTokenRouting::Refused(ResponseContinuation::from(challenge.response()))
}

fn route_presented_at<'request, 'trusted>(
    authorization: &'request RequestAuthorization,
    trusted_issuers: &[&'trusted TrustedIssuer],
    presented_at: NumericDate,
) -> BearerTokenRouting<'request, 'trusted> {
    let token = match authorization.bearer() {
        ControlFlow::Continue(token) => token,
        ControlFlow::Break(BearerChallenge::MissingCredentials) => {
            return BearerTokenRouting::Routed(RoutedBearerToken::Absent);
        }
        ControlFlow::Break(challenge) => return refused(challenge),
    };
    let JwtPresentation::Presented(presented) = PresentedJwt::present(token.as_str()) else {
        return refused(BearerChallenge::InvalidToken);
    };

    match presented.route(trusted_issuers.iter().copied()) {
        JwtRouting::Routed {
            addressee: trusted_issuer,
            jwt,
        } => BearerTokenRouting::Routed(RoutedBearerToken::Attributed(Box::new(
            AttributedBearerToken {
                jwt,
                presented_at,
                trusted_issuer,
            },
        ))),
        JwtRouting::Ambiguous(_) | JwtRouting::Misaddressed(_) | JwtRouting::UntrustedIssuer(_) => {
            refused(BearerChallenge::InvalidToken)
        }
    }
}

/// # Errors
///
/// Returns `RegisteredClaimsError` when the system clock cannot be read as a numeric date.
pub fn route_bearer_token<'request, 'trusted>(
    authorization: &'request RequestAuthorization,
    trusted_issuers: &[&'trusted TrustedIssuer],
) -> Result<BearerTokenRouting<'request, 'trusted>, RegisteredClaimsError> {
    NumericDate::from_system_time(SystemTime::now())
        .map(|presented_at| route_presented_at(authorization, trusted_issuers, presented_at))
}
