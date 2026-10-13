use std::sync::Arc;

use margaret_database::database::Database;
use margaret_jwt_verification_tests::constants::FIXTURE_AUDIENCE;
use margaret_oidc_sign_in::sign_in_callback_handler::SignInCallbackHandler;
use margaret_oidc_sign_in::sign_in_flow::SignInFlow;
use margaret_server_origin::route_url::RouteUrl;
use margaret_sessions::issued_sessions::IssuedSessions;
use margaret_sessions_tests::session_store::session_store;

use crate::constants::FIXTURE_LANDING;
use crate::fixture_admission::FixtureAdmission;

/// # Panics
///
/// Panics when the fixture landing is not a url.
#[must_use]
pub fn fixture_callback_handler(
    flow: Arc<SignInFlow>,
    admission: FixtureAdmission,
    database: Arc<Database>,
) -> SignInCallbackHandler<FixtureAdmission> {
    SignInCallbackHandler::create(
        flow,
        Arc::new(admission),
        Arc::new(IssuedSessions::host_only(
            database,
            session_store(),
            FIXTURE_AUDIENCE,
        )),
        RouteUrl::parse(FIXTURE_LANDING.to_string()).expect("the fixture landing is a url"),
    )
}
