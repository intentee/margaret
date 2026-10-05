use std::sync::Arc;

use margaret::framework::http::request::Request;
use margaret::framework::http::response::Response;
use margaret::framework::http_validation::request_input::RequestInput;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::oidc_provider::token_submission::TokenSubmission;
use margaret::framework::validation::validation_result::ValidationResult;

use crate::margaret::oidc_provider::IntrospectionEndpoint;

#[singleton]
#[responds_to_http(
    max_body_bytes = 8_388_608,
    method = "post",
    path = "/introspect",
    server = "identity"
)]
pub struct PostIntrospect {
    introspection_endpoint: Arc<IntrospectionEndpoint>,
}

impl PostIntrospect {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(introspection_endpoint: Arc<IntrospectionEndpoint>) -> anyhow::Result<Self> {
        Ok(Self {
            introspection_endpoint,
        })
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(
        &self,
        request: &Request,
        #[form_request(from = RequestInput::Form)] submission: ValidationResult<TokenSubmission>,
    ) -> anyhow::Result<Response> {
        Ok(self.introspection_endpoint.respond(request, submission))
    }
}
