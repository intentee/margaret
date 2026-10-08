use std::sync::Arc;

use uuid::Uuid;

use margaret::framework::database::database::Database;
use margaret_example::stores::milo_session::MILO_SESSION;
use margaret_example::stores::user_store::UserStore;
use margaret_example::system_clock::SystemClock;
use margaret_example_tests::seeded_blog::seeded_blog;

fn store(database: &Arc<Database>) -> UserStore {
    UserStore::create(Arc::new(SystemClock), Arc::clone(database))
        .expect("the user store is constructed")
}

#[tokio::test]
async fn finds_the_seeded_user_by_their_session() {
    let blog = seeded_blog().await;

    assert_eq!(
        store(&blog.database)
            .find_user_by_session(MILO_SESSION)
            .await
            .expect("the session is read")
            .map(|user| user.name),
        Some("Milo".to_string())
    );
}

#[tokio::test]
async fn finds_no_user_for_an_unknown_session() {
    let blog = seeded_blog().await;

    assert!(
        store(&blog.database)
            .find_user_by_session(Uuid::from_u128(1))
            .await
            .expect("the session is read")
            .is_none()
    );
}

#[tokio::test]
async fn starts_a_session_that_finds_its_user() {
    let blog = seeded_blog().await;
    let store = store(&blog.database);
    let session = store
        .start_session(Uuid::from_u128(3))
        .await
        .expect("the session is stored")
        .expect("a known user starts a session");

    assert_eq!(
        store
            .find_user_by_session(session)
            .await
            .expect("the session is read")
            .map(|user| user.name),
        Some("Milo".to_string())
    );
}

#[tokio::test]
async fn starts_no_session_for_an_unknown_user() {
    let blog = seeded_blog().await;

    assert!(
        store(&blog.database)
            .start_session(Uuid::from_u128(999))
            .await
            .expect("the session is attempted")
            .is_none()
    );
}
