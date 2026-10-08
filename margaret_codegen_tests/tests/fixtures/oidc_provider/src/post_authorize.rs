use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::http_validation::request_input::RequestInput;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret::framework::oidc_provider::authorization_request::AuthorizationRequest;
use margaret::framework::oidc_provider::end_user_authentication::EndUserAuthentication;
use margaret::framework::route_method::route_method::RouteMethod;
use margaret::framework::validation::validation_result::ValidationResult;

use crate::margaret::oidc_provider::AuthorizationEndpoint;

#[singleton]
#[responds_to_http(
    max_body_bytes = 4096,
    method = RouteMethod::Post,
    path = "/authorize",
    server = "public"
)]
pub struct PostAuthorize {
    authorization_endpoint: Arc<AuthorizationEndpoint>,
}

impl PostAuthorize {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(authorization_endpoint: Arc<AuthorizationEndpoint>) -> anyhow::Result<Self> {
        Ok(Self {
            authorization_endpoint,
        })
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub async fn respond(
        &self,
        #[form_request(from = RequestInput::Form)] request: ValidationResult<AuthorizationRequest>,
    ) -> anyhow::Result<Response> {
        Ok(
            match self
                .authorization_endpoint
                .authorize(request, &EndUserAuthentication::Anonymous)
                .await?
            {
                AuthorizationOutcome::AuthenticationRequired { return_to } => {
                    Response::text(401, return_to)
                }
                AuthorizationOutcome::ConsentRequired(consent) => {
                    Response::text(200, consent.id.to_string())
                }
                AuthorizationOutcome::Redirected(response)
                | AuthorizationOutcome::Rejected(response) => response,
            },
        )
    }
}
