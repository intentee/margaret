use chrono::Utc;
use uuid::Uuid;

use margaret::framework::active_record::change::Change;
use margaret_authorization_grants::refresh_family::RefreshFamily;
use margaret_authorization_grants::refresh_family_record::RefreshFamilyRecord;
use margaret_authorization_grants_tests::contract_refresh_family::contract_refresh_family;
use margaret_authorization_grants_tests::locked_refresh_token::locked_refresh_token;
use margaret_database::isolation::Isolation;
use margaret_database_tests::contract_token::contract_token;
use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider_tests::code_exchange::code_exchange;
use margaret_oidc_provider_tests::issued_code::issued_code;
use margaret_oidc_provider_tests::portal_credentials::PORTAL_CREDENTIALS;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_registered_claims::numeric_date::NumericDate;

const REPLAYED_CODE: &str =
    "the authorization code was already redeemed, so its tokens are revoked";

#[tokio::test]
async fn refuses_a_code_replayed_before_its_refresh_family_opens() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let AuthorizationOutcome::Redirected(redirect) = fixture.authorized(&portal_parameters()).await
    else {
        panic!("the portal is issued a code");
    };
    let exchange = code_exchange(&issued_code(&redirect));
    let expiry = NumericDate::from(Utc::now());
    let expiring_token = contract_token();

    RefreshFamilyRecord::open(
        &fixture.storage.database,
        Uuid::new_v4(),
        RefreshFamily {
            expires_at: expiry,
            ..contract_refresh_family()
        },
        expiring_token,
        NumericDate::new(expiry.seconds_since_epoch() - 1),
    )
    .await
    .expect("the database opens the expiring refresh family");

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
        locked_refresh_token(&holding, expiring_token).await,
        Change::Changed(_)
    ));

    let (answer, replayed) = tokio::join!(
        fixture.post_form("/token", &PORTAL_CREDENTIALS, &exchange),
        async {
            fixture.storage.administration.await_lock_waiters(1).await;

            let replayed = fixture
                .post_form("/token", &PORTAL_CREDENTIALS, &exchange)
                .await;

            holding.rollback().await.expect("the gate opens");

            replayed
        }
    );

    assert_eq!(replayed.body["error_description"], REPLAYED_CODE);
    assert_eq!(answer.status, 400);
    assert_eq!(answer.body["error"], "invalid_grant");
    assert_eq!(answer.body["error_description"], REPLAYED_CODE);

    fixture.stop().await;
}
