use margaret::framework::active_record::change::Change;
use margaret_authorization_grants::refresh_family_record::RefreshFamilyRecord;
use margaret_authorization_grants::refresh_token_lookup::RefreshTokenLookup;
use margaret_authorization_grants::refresh_token_record::RefreshTokenRecord;
use margaret_authorization_grants_tests::locked_refresh_token::locked_refresh_token;
use margaret_database::isolation::Isolation;
use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider_tests::issued_code::issued_code;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::portal_tokens::portal_tokens;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::refreshed_tokens::refreshed_tokens;
use margaret_token_digest::token_digest::TokenDigest;

#[tokio::test]
async fn refuses_the_rotated_token_of_a_family_revoked_during_its_rotation() {
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
    let RefreshTokenLookup::Current { family, .. } =
        RefreshTokenRecord::lookup(&fixture.storage.database, presented)
            .await
            .expect("the database finds refresh tokens")
    else {
        panic!("the refresh token is current");
    };
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

    let (rotated, revocation, ()) = tokio::join!(
        refreshed_tokens(&fixture, &refresh_token, None),
        async {
            fixture.storage.administration.await_lock_waiters(1).await;
            RefreshFamilyRecord::revoke(&fixture.storage.database, family).await
        },
        async {
            fixture.storage.administration.await_lock_waiters(2).await;
            holding.rollback().await.expect("the gate opens");
        }
    );

    revocation.expect("the database revokes the refresh family");

    let refused = refreshed_tokens(&fixture, &rotated.body["refresh_token"], None).await;

    assert_eq!(rotated.status, 200);
    assert_eq!(refused.status, 400);
    assert_eq!(
        refused.body["error_description"],
        "the refresh token is not known"
    );

    fixture.stop().await;
}
