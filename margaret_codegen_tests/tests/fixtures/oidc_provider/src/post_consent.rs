use std::sync::Arc;

use chrono::Utc;
use uuid::Uuid;

use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::oidc_provider::authenticated_end_user::AuthenticatedEndUser;
use margaret::framework::oidc_provider::consent_decision::ConsentDecision;
use margaret::framework::oidc_provider::consent_outcome::ConsentOutcome;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::margaret::oidc_provider::ConsentEndpoint;

#[singleton]
#[responds_to_http(method = RouteMethod::Post, path = "/consent", server = "public")]
pub struct PostConsent {
    consent_endpoint: Arc<ConsentEndpoint>,
}

impl PostConsent {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(consent_endpoint: Arc<ConsentEndpoint>) -> anyhow::Result<Self> {
        Ok(Self { consent_endpoint })
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub async fn respond(&self) -> anyhow::Result<Response> {
        Ok(
            match self
                .consent_endpoint
                .decide(
                    Uuid::nil(),
                    &AuthenticatedEndUser {
                        authenticated_at: Utc::now(),
                        subject: Uuid::nil(),
                    },
                    ConsentDecision::Approved,
                )
                .await?
            {
                ConsentOutcome::Redirected(response) => response,
                ConsentOutcome::Unknown => Response::not_found(),
            },
        )
    }
}
