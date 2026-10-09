use std::time::Duration;

use chrono::Utc;
use uuid::Uuid;

use margaret::framework::active_record::lookup::Lookup;
use margaret_database_tests::contract_instant::contract_instant;
use margaret_sessions::session::Session;
use margaret_sessions::session_lifetime_secs::SESSION_LIFETIME_SECS;
use margaret_sessions::session_record::SessionRecord;
use margaret_sessions_tests::started_with_sessions::started_with_sessions;
use margaret_token_digest::token_digest::TokenDigest;

#[tokio::test]
async fn finds_no_session_past_its_lifetime() {
    let started = started_with_sessions().await;
    let secret = TokenDigest::of("contract-secret");

    SessionRecord::open(
        &started.database,
        secret,
        Session {
            authenticated_at: Utc::now(),
            id: Uuid::new_v4(),
            subject: Uuid::new_v4(),
        },
        contract_instant(),
    )
    .await
    .expect("the session opens");

    assert!(matches!(
        SessionRecord::current(
            &started.database,
            secret,
            contract_instant().after(Duration::from_secs(u64::from(SESSION_LIFETIME_SECS))),
        )
        .await,
        Ok(Lookup::Missing)
    ));
}
