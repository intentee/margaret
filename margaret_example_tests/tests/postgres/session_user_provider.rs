use std::sync::Arc;

use chrono::Utc;
use uuid::Uuid;

use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret::framework::sessions::session::Session;
use margaret_example::auth::session_user_provider::SessionUserProvider;
use margaret_example::models::user::User;
use margaret_example_tests::seeded_blog::seeded_blog;

async fn inferred(session: Option<Session>) -> AuthenticatedUserOutcome<User> {
    let blog = seeded_blog().await;

    SessionUserProvider::create(Arc::clone(&blog.database))
        .expect("the provider is constructed")
        .infer_session_user(session)
        .await
        .expect("the session is read")
}

fn session_of(subject: Uuid) -> Session {
    Session {
        authenticated_at: Utc::now(),
        id: Uuid::from_u128(77),
        subject,
    }
}

#[tokio::test]
async fn finds_the_seeded_user_of_a_session() {
    assert!(matches!(
        inferred(Some(session_of(Uuid::from_u128(3)))).await,
        AuthenticatedUserOutcome::Authenticated(User { name, .. }) if name == "Milo"
    ));
}

#[tokio::test]
async fn finds_no_user_for_a_session_of_an_unknown_subject() {
    assert!(matches!(
        inferred(Some(session_of(Uuid::from_u128(1)))).await,
        AuthenticatedUserOutcome::Anonymous
    ));
}

#[tokio::test]
async fn finds_no_user_without_a_session() {
    assert!(matches!(
        inferred(None).await,
        AuthenticatedUserOutcome::Anonymous
    ));
}
