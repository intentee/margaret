use std::sync::Arc;

use margaret::framework::http::request::Request;
use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::oidc_provider::userinfo_authentication::UserinfoAuthentication;

use crate::margaret::oidc_provider::UserinfoEndpoint;

#[singleton]
#[responds_to_http(method = "get", path = "/userinfo", server = "public")]
pub struct GetUserinfo {
    userinfo_endpoint: Arc<UserinfoEndpoint>,
}

impl GetUserinfo {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(userinfo_endpoint: Arc<UserinfoEndpoint>) -> anyhow::Result<Self> {
        Ok(Self { userinfo_endpoint })
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(&self, request: &Request) -> anyhow::Result<Response> {
        Ok(match self.userinfo_endpoint.authenticate(request) {
            UserinfoAuthentication::Authenticated(grant) => self
                .userinfo_endpoint
                .answer(&grant, &serde_json::json!({}))?,
            UserinfoAuthentication::Refused(response) => response,
        })
    }
}
