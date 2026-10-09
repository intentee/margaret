use std::sync::Arc;

use margaret_database::database::Database;
use margaret_jwt_verification_tests::fixture_audience::FIXTURE_AUDIENCE;
use margaret_oidc_sign_in::sign_in_callback_handler::SignInCallbackHandler;
use margaret_oidc_sign_in::sign_in_flow::SignInFlow;
use margaret_sessions::issued_sessions::IssuedSessions;
use margaret_sessions_tests::session_store::session_store;

use crate::fixture_admission::FixtureAdmission;
use crate::fixture_landing::FIXTURE_LANDING;

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
        FIXTURE_LANDING.to_string(),
    )
}
