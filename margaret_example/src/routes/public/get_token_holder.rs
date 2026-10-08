use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::models::access_token_holder::AccessTokenHolder;

#[singleton]
#[responds_to_http(
    method = RouteMethod::Get,
    name = "get_token_holder",
    path = "/token-holder",
    server = "public"
)]
pub struct GetTokenHolder;

impl GetTokenHolder {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(
        &self,
        #[authenticated_user] holder: AccessTokenHolder,
    ) -> anyhow::Result<Response> {
        Ok(Response::text(
            200,
            format!("access token of {}", holder.subject),
        ))
    }
}
