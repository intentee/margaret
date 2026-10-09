use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;
use margaret::framework::sessions::started_session::StartedSession;

use crate::alice::ALICE;
use crate::margaret::sessions::IssuedSessions;
use crate::system_clock::SystemClock;

#[singleton]
#[responds_to_http(
    method = RouteMethod::Post,
    name = "post_alice_session",
    path = "/fixture/alice-session",
    server = "public"
)]
pub struct PostAliceSession {
    clock: Arc<SystemClock>,
    sessions: Arc<IssuedSessions>,
}

impl PostAliceSession {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(clock: Arc<SystemClock>, sessions: Arc<IssuedSessions>) -> anyhow::Result<Self> {
        Ok(Self { clock, sessions })
    }

    /// # Errors
    ///
    /// Returns an error when the session of Alice cannot be stored.
    #[process]
    pub async fn respond(&self) -> anyhow::Result<Response> {
        let StartedSession { cookie_changes, .. } =
            self.sessions.start(ALICE, self.clock.now()).await?;

        Ok(cookie_changes
            .cookies
            .iter()
            .fold(Response::text(200, "started"), Response::set_cookie))
    }
}
