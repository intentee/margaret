use margaret_authorization_grants::refresh_rotation::RefreshRotation;
use margaret_authorization_grants::refresh_token_record::RefreshTokenRecord;
use margaret_authorization_grants_tests::started_with_authorization_grants::started_with_authorization_grants;
use margaret_database_tests::contract_instant::contract_instant;
use margaret_database_tests::contract_token::contract_token;

#[tokio::test]
async fn refresh_rotation_finds_no_unknown_token() {
    let started = started_with_authorization_grants().await;
    let database = started.database.as_ref();

    assert_eq!(
        RefreshTokenRecord::rotate(
            database,
            contract_token(),
            contract_token(),
            contract_instant()
        )
        .await
        .expect("the database rotates refresh tokens"),
        RefreshRotation::Unknown
    );
}
