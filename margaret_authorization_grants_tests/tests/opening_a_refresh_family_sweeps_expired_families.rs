use uuid::Uuid;

use margaret_authorization_grants::family_opening::FamilyOpening;
use margaret_authorization_grants::refresh_family::RefreshFamily;
use margaret_authorization_grants::refresh_family_record::RefreshFamilyRecord;
use margaret_authorization_grants::refresh_token_lookup::RefreshTokenLookup;
use margaret_authorization_grants::refresh_token_record::RefreshTokenRecord;
use margaret_authorization_grants_tests::contract_refresh_family::contract_refresh_family;
use margaret_authorization_grants_tests::started_with_authorization_grants::started_with_authorization_grants;
use margaret_database_tests::contract_instant::contract_instant;
use margaret_database_tests::contract_token::contract_token;

#[tokio::test]
async fn opening_a_refresh_family_sweeps_expired_families() {
    let started = started_with_authorization_grants().await;
    let database = started.database.as_ref();
    let now = contract_instant();
    let expired_token = contract_token();

    assert_eq!(
        RefreshFamilyRecord::open(
            database,
            Uuid::new_v4(),
            RefreshFamily {
                expires_at: now,
                ..contract_refresh_family()
            },
            expired_token,
            now,
        )
        .await
        .expect("the database opens the expiring refresh family"),
        FamilyOpening::Opened
    );
    assert_eq!(
        RefreshFamilyRecord::open(
            database,
            Uuid::new_v4(),
            contract_refresh_family(),
            contract_token(),
            now
        )
        .await
        .expect("the database opens another refresh family"),
        FamilyOpening::Opened
    );
    assert_eq!(
        RefreshTokenRecord::lookup(database, expired_token)
            .await
            .expect("the database finds refresh tokens"),
        RefreshTokenLookup::Unknown
    );
}
