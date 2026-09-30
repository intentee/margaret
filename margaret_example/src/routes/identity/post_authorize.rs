use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::http_validation::request_input::RequestInput;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::oidc_provider::authorization_request::AuthorizationRequest;
use margaret::framework::validation::validation_result::ValidationResult;

use crate::margaret::oidc_provider::AuthorizationEndpoint;
use crate::margaret::routes::Routes;
use crate::margaret::views::Views;
use crate::models::user::User;
use crate::routes::identity::authorization_page::authorization_page;

#[singleton]
#[responds_to_http(
    max_body_bytes = 8_388_608,
    method = "post",
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
    /// Returns an error when the authorization cannot be stored or the consent page cannot be rendered.
    #[process]
    pub async fn respond(
        &self,
        routes: &Routes,
        views: &Views,
        #[authenticated_user] user: Option<User>,
        #[form_request(from = RequestInput::Form)] request: ValidationResult<AuthorizationRequest>,
    ) -> anyhow::Result<Response> {
        authorization_page(&self.authorization_endpoint, request, user, routes, views).await
    }
}
