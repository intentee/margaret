use margaret_authorization_grants::refresh_token_lookup::RefreshTokenLookup;
use margaret_authorization_grants::refresh_token_record::RefreshTokenRecord;
use margaret_authorization_grants_tests::started_with_authorization_grants::started_with_authorization_grants;
use margaret_database_tests::contract_token::contract_token;

#[tokio::test]
async fn refresh_token_lookup_finds_no_unknown_token() {
    let started = started_with_authorization_grants().await;
    let database = started.database.as_ref();

    assert_eq!(
        RefreshTokenRecord::lookup(database, contract_token())
            .await
            .expect("the database finds refresh tokens"),
        RefreshTokenLookup::Unknown
    );
}
