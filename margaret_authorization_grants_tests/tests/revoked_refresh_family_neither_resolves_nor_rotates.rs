use uuid::Uuid;

use margaret_authorization_grants::family_opening::FamilyOpening;
use margaret_authorization_grants::refresh_family_record::RefreshFamilyRecord;
use margaret_authorization_grants::refresh_family_revocation_record::RefreshFamilyRevocationRecord;
use margaret_authorization_grants::refresh_rotation::RefreshRotation;
use margaret_authorization_grants::refresh_token_lookup::RefreshTokenLookup;
use margaret_authorization_grants::refresh_token_record::RefreshTokenRecord;
use margaret_authorization_grants_tests::contract_refresh_family::contract_refresh_family;
use margaret_authorization_grants_tests::started_with_authorization_grants::started_with_authorization_grants;
use margaret_database_tests::contract_instant::contract_instant;
use margaret_database_tests::contract_token::contract_token;

#[tokio::test]
async fn revoked_refresh_family_neither_resolves_nor_rotates() {
    let started = started_with_authorization_grants().await;
    let database = started.database.as_ref();
    let family = Uuid::new_v4();
    let presented = contract_token();
    let next = contract_token();
    let now = contract_instant();

    assert_eq!(
        RefreshFamilyRecord::open(database, family, contract_refresh_family(), presented, now)
            .await
            .expect("the database opens the refresh family"),
        FamilyOpening::Opened
    );

    RefreshFamilyRevocationRecord::revoke(database, family, now)
        .await
        .expect("the database revokes the refresh family");

    assert_eq!(
        RefreshTokenRecord::lookup(database, presented)
            .await
            .expect("the database finds refresh tokens"),
        RefreshTokenLookup::Unknown
    );
    assert_eq!(
        RefreshTokenRecord::rotate(database, presented, next, now)
            .await
            .expect("the database rotates refresh tokens"),
        RefreshRotation::Unknown
    );
    assert_eq!(
        RefreshTokenRecord::lookup(database, next)
            .await
            .expect("the database finds refresh tokens"),
        RefreshTokenLookup::Unknown
    );
}
