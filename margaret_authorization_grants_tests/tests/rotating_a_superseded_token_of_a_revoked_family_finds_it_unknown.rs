use uuid::Uuid;

use margaret_authorization_grants::family_opening::FamilyOpening;
use margaret_authorization_grants::refresh_family_record::RefreshFamilyRecord;
use margaret_authorization_grants::refresh_family_revocation_record::RefreshFamilyRevocationRecord;
use margaret_authorization_grants::refresh_rotation::RefreshRotation;
use margaret_authorization_grants::refresh_token_record::RefreshTokenRecord;
use margaret_authorization_grants_tests::contract_refresh_family::contract_refresh_family;
use margaret_authorization_grants_tests::started_with_authorization_grants::started_with_authorization_grants;
use margaret_database_tests::contract_instant::contract_instant;
use margaret_database_tests::contract_token::contract_token;

#[tokio::test]
async fn rotating_a_superseded_token_of_a_revoked_family_finds_it_unknown() {
    let started = started_with_authorization_grants().await;
    let database = started.database.as_ref();
    let family = Uuid::new_v4();
    let presented = contract_token();
    let now = contract_instant();

    assert_eq!(
        RefreshFamilyRecord::open(database, family, contract_refresh_family(), presented, now)
            .await
            .expect("the database opens the refresh family"),
        FamilyOpening::Opened
    );
    assert_eq!(
        RefreshTokenRecord::rotate(database, presented, contract_token(), now)
            .await
            .expect("the database rotates refresh tokens"),
        RefreshRotation::Rotated
    );
    RefreshFamilyRevocationRecord::revoke(database, family, now)
        .await
        .expect("the database revokes the refresh family");

    assert_eq!(
        RefreshTokenRecord::rotate(database, presented, contract_token(), now)
            .await
            .expect("the database rotates refresh tokens"),
        RefreshRotation::Unknown
    );
}
