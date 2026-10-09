use std::time::Duration;

use chrono::Utc;
use uuid::Uuid;

use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret_database_tests::contract_instant::contract_instant;
use margaret_sessions::session::Session;
use margaret_sessions::session_lifetime_secs::SESSION_LIFETIME_SECS;
use margaret_sessions::session_record::SessionRecord;
use margaret_sessions_tests::started_with_sessions::started_with_sessions;
use margaret_token_digest::token_digest::TokenDigest;

fn session() -> Session {
    Session {
        authenticated_at: Utc::now(),
        id: Uuid::new_v4(),
        subject: Uuid::new_v4(),
    }
}

#[tokio::test]
async fn opening_a_session_sweeps_expired_sessions() {
    let started = started_with_sessions().await;
    let expired = TokenDigest::of("expired-secret");

    SessionRecord::open(&started.database, expired, session(), contract_instant())
        .await
        .expect("the first session opens");
    SessionRecord::open(
        &started.database,
        TokenDigest::of("later-secret"),
        session(),
        contract_instant().after(Duration::from_secs(u64::from(SESSION_LIFETIME_SECS))),
    )
    .await
    .expect("the later session opens");

    assert!(matches!(
        SessionRecord::query()
            .secret
            .eq(expired.as_bytes().to_vec())
            .find(started.database.as_ref())
            .await,
        Ok(Lookup::Missing)
    ));
}
