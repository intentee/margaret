use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::http_validation::request_input::RequestInput;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::oidc_provider::consent_outcome::ConsentOutcome;
use margaret::framework::route_method::route_method::RouteMethod;
use margaret::framework::validation::validation_result::ValidationResult;

use crate::forms::consent_form::ConsentForm;
use crate::margaret::oidc_provider::ConsentEndpoint;
use crate::models::user::User;

#[singleton]
#[responds_to_http(
    max_body_bytes = 8_388_608,
    method = RouteMethod::Post,
    name = "post_authorization_consent",
    path = "/authorize/consent",
    server = "identity"
)]
pub struct PostAuthorizationConsent {
    consent_endpoint: Arc<ConsentEndpoint>,
}

impl PostAuthorizationConsent {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(consent_endpoint: Arc<ConsentEndpoint>) -> anyhow::Result<Self> {
        Ok(Self { consent_endpoint })
    }

    /// # Errors
    ///
    /// Returns an error when the consent decision cannot be stored.
    #[process]
    pub async fn respond(
        &self,
        #[authenticated_user] user: User,
        #[form_request(from = RequestInput::Form)] form: ValidationResult<ConsentForm>,
    ) -> anyhow::Result<Response> {
        let ValidationResult::Valid(ConsentForm { decision, id }) = form else {
            return Ok(Response::text(422, "The consent decision is malformed"));
        };

        Ok(
            match self
                .consent_endpoint
                .decide(id, &user.end_user(), decision.decision())
                .await?
            {
                ConsentOutcome::Redirected(response) => response,
                ConsentOutcome::Unknown => Response::text(
                    404,
                    "The authorization request is unknown or was already decided",
                ),
            },
        )
    }
}
