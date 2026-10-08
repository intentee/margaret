use uuid::Uuid;

use margaret_authorization_grants::family_opening::FamilyOpening;
use margaret_authorization_grants::refresh_family_record::RefreshFamilyRecord;
use margaret_authorization_grants::refresh_token_lookup::RefreshTokenLookup;
use margaret_authorization_grants::refresh_token_record::RefreshTokenRecord;
use margaret_authorization_grants_tests::contract_refresh_family::contract_refresh_family;
use margaret_authorization_grants_tests::started_with_authorization_grants::started_with_authorization_grants;
use margaret_database_tests::contract_instant::contract_instant;
use margaret_database_tests::contract_token::contract_token;

#[tokio::test]
async fn opened_refresh_family_resolves_its_first_token() {
    let started = started_with_authorization_grants().await;
    let database = started.database.as_ref();
    let family = Uuid::new_v4();
    let record = contract_refresh_family();
    let first_token = contract_token();

    assert_eq!(
        RefreshFamilyRecord::open(
            database,
            family,
            record.clone(),
            first_token,
            contract_instant()
        )
        .await
        .expect("the database opens the refresh family"),
        FamilyOpening::Opened
    );
    assert_eq!(
        RefreshTokenRecord::lookup(database, first_token)
            .await
            .expect("the database finds refresh tokens"),
        RefreshTokenLookup::Current { family, record }
    );
}
