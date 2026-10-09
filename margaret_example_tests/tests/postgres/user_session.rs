use std::sync::Arc;

use chrono::Utc;
use uuid::Uuid;

use margaret::framework::active_record::creation::Creation;
use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret_example::auth::session_user_provider::SessionUserProvider;
use margaret_example::forms::session_cookie::SessionCookie;
use margaret_example::models::user::User;
use margaret_example::models::user_session::UserSession;
use margaret_example_tests::seeded_blog::seeded_blog;

#[tokio::test]
async fn starts_a_session_that_finds_its_user() {
    let blog = seeded_blog().await;
    let Creation::Created(session) =
        UserSession::start(&blog.database, Uuid::from_u128(3), Utc::now())
            .await
            .expect("the session is stored")
    else {
        panic!("a known user starts a session");
    };

    assert!(matches!(
        SessionUserProvider::create(Arc::clone(&blog.database))
            .expect("the provider is constructed")
            .infer_session_user(SessionCookie {
                session: Some(session.id.to_string()),
            })
            .await
            .expect("the session is read"),
        AuthenticatedUserOutcome::Authenticated(User { name, .. }) if name == "Milo"
    ));
}

#[tokio::test]
async fn starts_no_session_for_an_unknown_user() {
    let blog = seeded_blog().await;

    assert!(matches!(
        UserSession::start(&blog.database, Uuid::from_u128(999), Utc::now())
            .await
            .expect("the session is attempted"),
        Creation::Refused
    ));
}
