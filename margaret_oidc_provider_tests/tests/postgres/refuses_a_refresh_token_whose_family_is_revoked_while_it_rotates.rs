use margaret::framework::active_record::model::Model;
use margaret::framework::active_record::removal::Removal;
use margaret_authorization_grants::refresh_family_record::RefreshFamilyRecord;
use margaret_authorization_grants::refresh_token_lookup::RefreshTokenLookup;
use margaret_authorization_grants::refresh_token_record::RefreshTokenRecord;
use margaret_database::isolation::Isolation;
use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider_tests::issued_code::issued_code;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::portal_tokens::portal_tokens;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::refreshed_tokens::refreshed_tokens;
use margaret_token_digest::token_digest::TokenDigest;

#[tokio::test]
async fn refuses_a_refresh_token_whose_family_is_revoked_while_it_rotates() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let AuthorizationOutcome::Redirected(redirect) = fixture.authorized(&portal_parameters()).await
    else {
        panic!("the portal is issued a code");
    };
    let refresh_token =
        portal_tokens(&fixture, &issued_code(&redirect)).await.body["refresh_token"].clone();
    let RefreshTokenLookup::Current { family, .. } = RefreshTokenRecord::lookup(
        &fixture.storage.database,
        TokenDigest::of(
            refresh_token
                .as_str()
                .expect("the refresh token is a string"),
        ),
    )
    .await
    .expect("the database finds refresh tokens") else {
        panic!("the refresh token is current");
    };
    let mut gate = fixture
        .storage
        .database
        .connection()
        .await
        .expect("the revoking connection is checked out");
    let revoking = gate
        .transaction(Isolation::ReadCommitted)
        .await
        .expect("the revoking transaction begins");

    assert!(matches!(
        RefreshFamilyRecord::query()
            .id
            .eq(family)
            .delete(&revoking)
            .await
            .expect("the revoking transaction forgets the family"),
        Removal::Removed(_)
    ));

    let (refused, ()) = tokio::join!(refreshed_tokens(&fixture, &refresh_token, None), async {
        fixture.storage.administration.await_lock_waiters(1).await;
        revoking.commit().await.expect("the revocation commits");
    });

    assert_eq!(refused.status, 400);
    assert_eq!(
        refused.body["error_description"],
        "the refresh token is not known"
    );

    fixture.stop().await;
}
