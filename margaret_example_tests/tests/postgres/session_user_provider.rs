use std::sync::Arc;

use uuid::Uuid;

use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret_example::auth::session_user_provider::SessionUserProvider;
use margaret_example::forms::session_cookie::SessionCookie;
use margaret_example::milo_session::MILO_SESSION;
use margaret_example::models::user::User;
use margaret_example_tests::seeded_blog::seeded_blog;

async fn inferred(session: Uuid) -> AuthenticatedUserOutcome<User> {
    let blog = seeded_blog().await;

    SessionUserProvider::create(Arc::clone(&blog.database))
        .expect("the provider is constructed")
        .infer_session_user(SessionCookie {
            session: Some(session.to_string()),
        })
        .await
        .expect("the session is read")
}

#[tokio::test]
async fn finds_the_seeded_user_by_their_session() {
    assert!(matches!(
        inferred(MILO_SESSION).await,
        AuthenticatedUserOutcome::Authenticated(User { name, .. }) if name == "Milo"
    ));
}

#[tokio::test]
async fn finds_no_user_for_an_unknown_session() {
    assert!(matches!(
        inferred(Uuid::from_u128(1)).await,
        AuthenticatedUserOutcome::Anonymous
    ));
}
