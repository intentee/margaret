use chrono::Utc;

use margaret::framework::active_record::change::Change;
use margaret_authorization_grants::refresh_rotation::RefreshRotation;
use margaret_authorization_grants::refresh_token_record::RefreshTokenRecord;
use margaret_authorization_grants_tests::locked_refresh_token::locked_refresh_token;
use margaret_database::isolation::Isolation;
use margaret_database_tests::contract_token::contract_token;
use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider_tests::issued_code::issued_code;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::portal_tokens::portal_tokens;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::refreshed_tokens::refreshed_tokens;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_token_digest::token_digest::TokenDigest;

#[tokio::test]
async fn refuses_a_refresh_token_rotated_concurrently() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let AuthorizationOutcome::Redirected(redirect) = fixture.authorized(&portal_parameters()).await
    else {
        panic!("the portal is issued a code");
    };
    let refresh_token =
        portal_tokens(&fixture, &issued_code(&redirect)).await.body["refresh_token"].clone();
    let presented = TokenDigest::of(
        refresh_token
            .as_str()
            .expect("the refresh token is a string"),
    );
    let mut gate = fixture
        .storage
        .database
        .connection()
        .await
        .expect("the gate connection is checked out");
    let holding = gate
        .transaction(Isolation::ReadCommitted)
        .await
        .expect("the gate transaction begins");

    assert!(matches!(
        locked_refresh_token(&holding, presented).await,
        Change::Changed(_)
    ));

    let (competing, raced, ()) = tokio::join!(
        RefreshTokenRecord::rotate(
            &fixture.storage.database,
            presented,
            contract_token(),
            NumericDate::from(Utc::now()),
        ),
        async {
            fixture.storage.administration.await_lock_waiters(1).await;
            refreshed_tokens(&fixture, &refresh_token, None).await
        },
        async {
            fixture.storage.administration.await_lock_waiters(2).await;
            holding.rollback().await.expect("the gate opens");
        }
    );

    assert_eq!(
        competing.expect("the database rotates refresh tokens"),
        RefreshRotation::Rotated
    );
    assert_eq!(raced.status, 400);
    assert_eq!(
        raced.body["error_description"],
        "the refresh token was already rotated, so its family is revoked"
    );
    assert_eq!(
        refreshed_tokens(&fixture, &refresh_token, None).await.body["error_description"],
        "the refresh token is not known"
    );

    fixture.stop().await;
}
