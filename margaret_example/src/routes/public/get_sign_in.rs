use std::collections::BTreeSet;
use std::sync::Arc;

use url::Url;

use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::oauth_vocabulary::scope::Scope;
use margaret::framework::oidc_sign_in::sign_in_beginning::SignInBeginning;
use margaret::framework::oidc_sign_in::sign_in_request::SignInRequest;

use crate::auth::profile_scope::PROFILE_SCOPE;
use crate::margaret::oauth_clients::blog::SignInFlow;
use crate::margaret::routes::Routes;

#[singleton]
#[responds_to_http(method = "get", path = "/sign-in", server = "public")]
pub struct GetSignIn {
    profile_scope: Scope,
    sign_in_flow: Arc<SignInFlow>,
}

impl GetSignIn {
    /// # Errors
    ///
    /// Returns an error when the profile scope is malformed.
    #[constructor]
    pub fn create(sign_in_flow: Arc<SignInFlow>) -> anyhow::Result<Self> {
        Ok(Self {
            profile_scope: PROFILE_SCOPE.parse()?,
            sign_in_flow,
        })
    }

    /// # Errors
    ///
    /// Returns an error when the callback url is malformed or the sign-in transaction cannot be
    /// signed.
    #[process]
    pub async fn respond(&self, routes: &Routes) -> anyhow::Result<Response> {
        Ok(
            match self
                .sign_in_flow
                .begin(SignInRequest {
                    callback: Url::parse(&routes.public.get_sign_in_callback.url())?,
                    scopes: BTreeSet::from([self.profile_scope.clone()]),
                })
                .await?
            {
                SignInBeginning::Redirected(response) => response,
                SignInBeginning::Unavailable(unavailability) => {
                    Response::text(503, unavailability.to_string())
                }
            },
        )
    }
}
