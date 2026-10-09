use uuid::Uuid;

use margaret_authorization_server_client_tests::secret_basic_authentication::secret_basic_authentication;
use margaret_handler_error::handler_error::HandlerError;
use margaret_http::head_handler::HeadHandler;
use margaret_oidc_sign_in::sign_in_beginning::SignInBeginning;
use margaret_oidc_sign_in_tests::begun_sign_in::BegunSignIn;
use margaret_oidc_sign_in_tests::fixture_admission::FixtureAdmission;
use margaret_oidc_sign_in_tests::fixture_callback_handler::fixture_callback_handler;
use margaret_oidc_sign_in_tests::sign_in_fixture::SignInFixture;
use margaret_oidc_sign_in_tests::signed_in_callback::signed_in_callback;
use margaret_sessions_tests::started_with_sessions::started_with_sessions;

#[tokio::test]
async fn reports_a_session_that_cannot_start() {
    let fixture = SignInFixture::start(secret_basic_authentication()).await;
    let started = started_with_sessions().await;
    let SignInBeginning::Redirected(beginning) = fixture.flow.begin().await else {
        panic!("the sign-in redirects to the authorization endpoint");
    };
    let callback = signed_in_callback(&fixture, &BegunSignIn::of(&beginning));
    let held_connection = started
        .database
        .connection()
        .await
        .expect("the idle connection of the pool is checked out");

    started.administration.make_unreachable().await;

    let answered = fixture_callback_handler(
        fixture.flow.clone(),
        FixtureAdmission::Admitting(Uuid::from_u128(9)),
        started.database.clone(),
    )
    .handle(&callback)
    .await;

    fixture.server.stop().await;
    drop(held_connection);

    assert!(matches!(answered, Err(HandlerError::Consumer { .. })));
}
