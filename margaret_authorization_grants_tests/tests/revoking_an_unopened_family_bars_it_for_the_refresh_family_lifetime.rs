use uuid::Uuid;

use margaret_authorization_grants::family_opening::FamilyOpening;
use margaret_authorization_grants::refresh_family_lifetime::REFRESH_FAMILY_LIFETIME;
use margaret_authorization_grants::refresh_family_record::RefreshFamilyRecord;
use margaret_authorization_grants::refresh_family_revocation_record::RefreshFamilyRevocationRecord;
use margaret_authorization_grants_tests::contract_refresh_family::contract_refresh_family;
use margaret_authorization_grants_tests::started_with_authorization_grants::started_with_authorization_grants;
use margaret_database_tests::contract_instant::contract_instant;
use margaret_database_tests::contract_token::contract_token;
use margaret_registered_claims::numeric_date::NumericDate;

#[tokio::test]
async fn revoking_an_unopened_family_bars_it_for_the_refresh_family_lifetime() {
    let started = started_with_authorization_grants().await;
    let database = started.database.as_ref();
    let revoked_at = contract_instant();
    let family = Uuid::new_v4();
    let last_moment = NumericDate::new(
        revoked_at
            .after(REFRESH_FAMILY_LIFETIME)
            .seconds_since_epoch()
            - 1,
    );

    RefreshFamilyRevocationRecord::revoke(database, family, revoked_at)
        .await
        .expect("the database revokes the unopened family");

    assert_eq!(
        RefreshFamilyRecord::open(
            database,
            family,
            contract_refresh_family(),
            contract_token(),
            last_moment
        )
        .await
        .expect("the database opens refresh families"),
        FamilyOpening::Revoked
    );
}
