use std::sync::Arc;

use chrono::Utc;
use uuid::Uuid;

use margaret::framework::http::redirect::Redirect;
use margaret::framework::http::response_continuation::ResponseContinuation;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::margaret::routes::Routes;
use crate::margaret::sessions::IssuedSessions;

#[singleton]
#[responds_to_http(method = RouteMethod::Post, path = "/sign-in", server = "public")]
pub struct PostSignIn {
    sessions: Arc<IssuedSessions>,
}

impl PostSignIn {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(sessions: Arc<IssuedSessions>) -> anyhow::Result<Self> {
        Ok(Self { sessions })
    }

    /// # Errors
    ///
    /// Returns an error when the session cannot be started.
    #[process]
    pub async fn respond(&self, routes: &Routes) -> anyhow::Result<ResponseContinuation> {
        Ok(self
            .sessions
            .start(Uuid::new_v4(), Utc::now())
            .await?
            .cookie_changes
            .precede(ResponseContinuation::from(Redirect::see_other(
                routes.public.get_profile.url(),
            ))))
    }
}
