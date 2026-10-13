use uuid::Uuid;

use margaret_authorization_grants::refresh_family::RefreshFamily;
use margaret_authorization_grants::refresh_rotation::RefreshRotation;
use margaret_authorization_grants::refresh_token_lookup::RefreshTokenLookup;
use margaret_authorization_grants::refresh_token_record::RefreshTokenRecord;
use margaret_authorization_grants_tests::contract_refresh_family::contract_refresh_family;
use margaret_authorization_grants_tests::opened_refresh_family::opened_refresh_family;
use margaret_authorization_grants_tests::started_with_authorization_grants::started_with_authorization_grants;
use margaret_database_tests::contract_instant::contract_instant;
use margaret_database_tests::contract_token::contract_token;

#[tokio::test]
async fn rotating_a_refresh_token_sweeps_expired_families() {
    let started = started_with_authorization_grants().await;
    let database = started.database.as_ref();
    let now = contract_instant();
    let presented = contract_token();
    let expired_token = contract_token();

    opened_refresh_family(
        database,
        Uuid::new_v4(),
        contract_refresh_family(),
        presented,
        now,
    )
    .await;
    opened_refresh_family(
        database,
        Uuid::new_v4(),
        RefreshFamily {
            expires_at: now,
            ..contract_refresh_family()
        },
        expired_token,
        now,
    )
    .await;
    assert_eq!(
        RefreshTokenRecord::rotate(database, presented, contract_token(), now)
            .await
            .expect("the database rotates refresh tokens"),
        RefreshRotation::Rotated
    );
    assert_eq!(
        RefreshTokenRecord::lookup(database, expired_token)
            .await
            .expect("the database finds refresh tokens"),
        RefreshTokenLookup::Unknown
    );
}
