use std::sync::Arc;

use serde::Serialize;

use margaret::framework::http::request::Request;
use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::oauth_vocabulary::scope::Scope;
use margaret::framework::oidc_provider::userinfo_authentication::UserinfoAuthentication;
use margaret::framework::oidc_provider::userinfo_grant::UserinfoGrant;

use crate::auth::profile_scope::PROFILE_SCOPE;
use crate::margaret::oidc_provider::UserinfoEndpoint;
use crate::stores::user_store::UserStore;

#[derive(Serialize)]
struct ProfileClaims {
    name: String,
}

#[singleton]
#[responds_to_http(method = "get", path = "/userinfo", server = "identity")]
pub struct GetUserinfo {
    profile_scope: Scope,
    userinfo_endpoint: Arc<UserinfoEndpoint>,
    users: Arc<UserStore>,
}

impl GetUserinfo {
    /// # Errors
    ///
    /// Returns an error when the profile scope is malformed.
    #[constructor]
    pub fn create(
        userinfo_endpoint: Arc<UserinfoEndpoint>,
        users: Arc<UserStore>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            profile_scope: PROFILE_SCOPE.parse()?,
            userinfo_endpoint,
            users,
        })
    }

    /// # Errors
    ///
    /// Returns an error when the profile claims cannot be merged with the subject.
    #[process]
    pub fn respond(&self, request: &Request) -> anyhow::Result<Response> {
        Ok(match self.userinfo_endpoint.authenticate(request) {
            UserinfoAuthentication::Authenticated(grant) => self.answer(&grant)?,
            UserinfoAuthentication::Refused(response) => response,
        })
    }

    fn answer(&self, grant: &UserinfoGrant) -> anyhow::Result<Response> {
        Ok(if grant.scopes.contains(&self.profile_scope) {
            match self.users.find_user_name(grant.subject) {
                Some(name) => self
                    .userinfo_endpoint
                    .answer(grant, &ProfileClaims { name })?,
                None => Response::text(404, "The signed-in user is unknown"),
            }
        } else {
            self.userinfo_endpoint
                .answer(grant, &serde_json::Map::new())?
        })
    }
}
