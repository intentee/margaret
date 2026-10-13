use std::sync::Arc;
use std::time::Duration;

use oauth2::TokenResponse;
use oauth2::basic::BasicTokenResponse;

use margaret_authorization_server_client::endpoint_outcome::EndpointOutcome;

use crate::acquired_token::AcquiredToken;
use crate::acquired_token_refresh_margin::ACQUIRED_TOKEN_REFRESH_MARGIN;

#[derive(Clone)]
pub(crate) struct CachedAcquisition {
    pub(crate) acquired: AcquiredToken,
    pub(crate) reusable_for: Duration,
}

impl CachedAcquisition {
    pub(crate) fn of(outcome: EndpointOutcome<BasicTokenResponse>) -> Self {
        match outcome {
            EndpointOutcome::Answered(response) => Self {
                acquired: AcquiredToken::Acquired(response.access_token().clone()),
                reusable_for: response
                    .expires_in()
                    .and_then(|lifetime| lifetime.checked_sub(ACQUIRED_TOKEN_REFRESH_MARGIN))
                    .unwrap_or(Duration::ZERO),
            },
            EndpointOutcome::Refused(refusal) => Self {
                acquired: AcquiredToken::Refused(refusal),
                reusable_for: Duration::ZERO,
            },
            EndpointOutcome::Unavailable(unavailability) => Self {
                acquired: AcquiredToken::Unavailable(Arc::new(unavailability)),
                reusable_for: Duration::ZERO,
            },
        }
    }
}
