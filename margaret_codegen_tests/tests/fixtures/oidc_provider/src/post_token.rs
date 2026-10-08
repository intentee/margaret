use std::sync::Arc;

use margaret::framework::http::request::Request;
use margaret::framework::http::response::Response;
use margaret::framework::http_validation::request_input::RequestInput;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::oidc_provider::token_request::TokenRequest;
use margaret::framework::route_method::route_method::RouteMethod;
use margaret::framework::validation::validation_result::ValidationResult;

use crate::margaret::oidc_provider::TokenEndpoint;

#[singleton]
#[responds_to_http(
    max_body_bytes = 16384,
    method = RouteMethod::Post,
    path = "/token",
    server = "public"
)]
pub struct PostToken {
    token_endpoint: Arc<TokenEndpoint>,
}

impl PostToken {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(token_endpoint: Arc<TokenEndpoint>) -> anyhow::Result<Self> {
        Ok(Self { token_endpoint })
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub async fn respond(
        &self,
        request: &Request,
        #[form_request(from = RequestInput::Form)] token_request: ValidationResult<TokenRequest>,
    ) -> anyhow::Result<Response> {
        Ok(self.token_endpoint.respond(request, token_request).await?)
    }
}
