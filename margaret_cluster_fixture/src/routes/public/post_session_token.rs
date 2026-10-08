use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::margaret::jwks::JwksSecretStore;
use crate::models::user::User;
use crate::routes::public::session_refresh_token::SessionRefreshToken;
use crate::system_clock::SystemClock;

#[singleton]
#[responds_to_http(
    method = RouteMethod::Post,
    name = "post_session_token",
    path = "/session-token",
    server = "public"
)]
pub struct PostSessionToken {
    clock: Arc<SystemClock>,
    secret_store: Arc<JwksSecretStore>,
}

impl PostSessionToken {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(
        clock: Arc<SystemClock>,
        secret_store: Arc<JwksSecretStore>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            clock,
            secret_store,
        })
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(&self, #[authenticated_user] user: User) -> anyhow::Result<Response> {
        Ok(Response::json(
            200,
            &SessionRefreshToken {
                refresh_token: self
                    .secret_store
                    .issue_refresh_token(user.id, self.clock.now())
                    .signed_claims,
            },
        ))
    }
}
