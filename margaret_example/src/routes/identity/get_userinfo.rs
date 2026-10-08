use std::sync::Arc;

use serde::Serialize;

use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret::framework::database::database::Database;
use margaret::framework::http::request::Request;
use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::oidc_provider::userinfo_authentication::UserinfoAuthentication;
use margaret::framework::oidc_provider::userinfo_grant::UserinfoGrant;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::auth::profile_scope::PROFILE_SCOPE;
use crate::margaret::oidc_provider::UserinfoEndpoint;
use crate::models::user_account::UserAccount;

#[derive(Serialize)]
struct ProfileClaims {
    name: String,
}

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/userinfo", server = "identity")]
pub struct GetUserinfo {
    database: Arc<Database>,
    userinfo_endpoint: Arc<UserinfoEndpoint>,
}

impl GetUserinfo {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(
        database: Arc<Database>,
        userinfo_endpoint: Arc<UserinfoEndpoint>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            database,
            userinfo_endpoint,
        })
    }

    /// # Errors
    ///
    /// Returns an error when the profile claims cannot be merged with the subject.
    #[process]
    pub async fn respond(&self, request: &Request) -> anyhow::Result<Response> {
        Ok(match self.userinfo_endpoint.authenticate(request) {
            UserinfoAuthentication::Authenticated(grant) => self.answer(&grant).await?,
            UserinfoAuthentication::Refused(response) => response,
        })
    }

    async fn answer(&self, grant: &UserinfoGrant) -> anyhow::Result<Response> {
        Ok(
            if grant
                .scopes
                .iter()
                .any(|scope| scope.as_str() == PROFILE_SCOPE)
            {
                match UserAccount::query()
                    .id
                    .eq(grant.subject)
                    .find(self.database.as_ref())
                    .await?
                {
                    Lookup::Found(UserAccount { name, .. }) => self
                        .userinfo_endpoint
                        .answer(grant, &ProfileClaims { name })?,
                    Lookup::Missing => Response::text(404, "The signed-in user is unknown"),
                }
            } else {
                self.userinfo_endpoint
                    .answer(grant, &serde_json::Map::new())?
            },
        )
    }
}
