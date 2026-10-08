use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::http_validation::request_input::RequestInput;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::oidc_provider::authorization_request::AuthorizationRequest;
use margaret::framework::route_method::route_method::RouteMethod;
use margaret::framework::validation::validation_result::ValidationResult;

use crate::margaret::oidc_provider::AuthorizationEndpoint;
use crate::models::user::User;
use crate::routes::identity::authorization_response::authorization_response;

#[singleton]
#[responds_to_http(
    max_body_bytes = 8_388_608,
    method = RouteMethod::Post,
    path = "/authorize",
    server = "identity"
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
    /// Returns an error when the pending authorization cannot be stored.
    #[process]
    pub async fn respond(
        &self,
        #[authenticated_user] user: Option<User>,
        #[form_request(from = RequestInput::Form)] request: ValidationResult<AuthorizationRequest>,
    ) -> anyhow::Result<Response> {
        authorization_response(&self.authorization_endpoint, request, user).await
    }
}
