use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::oidc_sign_in::sign_in_beginning::SignInBeginning;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::margaret::oauth_clients::blog::SignInFlow;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/sign-in", server = "public")]
pub struct GetSignIn {
    sign_in_flow: Arc<SignInFlow>,
}

impl GetSignIn {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(sign_in_flow: Arc<SignInFlow>) -> anyhow::Result<Self> {
        Ok(Self { sign_in_flow })
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub async fn respond(&self) -> anyhow::Result<Response> {
        Ok(match self.sign_in_flow.begin().await {
            SignInBeginning::Redirected(response) => response,
            SignInBeginning::Unavailable(unavailability) => {
                Response::text(503, unavailability.to_string())
            }
        })
    }
}
