use uuid::Uuid;

use margaret_authorization_grants::refresh_token_lookup::RefreshTokenLookup;
use margaret_authorization_grants::refresh_token_record::RefreshTokenRecord;
use margaret_authorization_grants_tests::contract_refresh_family::contract_refresh_family;
use margaret_authorization_grants_tests::opened_refresh_family::opened_refresh_family;
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

    opened_refresh_family(
        database,
        family,
        record.clone(),
        first_token,
        contract_instant(),
    )
    .await;
    assert_eq!(
        RefreshTokenRecord::lookup(database, first_token)
            .await
            .expect("the database finds refresh tokens"),
        RefreshTokenLookup::Current { family, record }
    );
}
