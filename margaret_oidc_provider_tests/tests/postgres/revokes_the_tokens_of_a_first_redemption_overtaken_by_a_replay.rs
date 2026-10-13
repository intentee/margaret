use serde_json::json;

use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider_tests::code_exchange::code_exchange;
use margaret_oidc_provider_tests::issued_code::issued_code;
use margaret_oidc_provider_tests::portal_credentials::PORTAL_CREDENTIALS;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_sql_identifier::qualified_table::qualified_table;
use margaret_sql_identifier::table_namespace::TableNamespace;

const REPLAYED_CODE: &str =
    "the authorization code was already redeemed, so its tokens are revoked";

#[tokio::test]
async fn revokes_the_tokens_of_a_first_redemption_overtaken_by_a_replay() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let AuthorizationOutcome::Redirected(redirect) = fixture.authorized(&portal_parameters()).await
    else {
        panic!("the portal is issued a code");
    };
    let exchange = code_exchange(&issued_code(&redirect));
    let gate = fixture.storage.administration.client().await;

    gate.batch_execute(&format!(
        "BEGIN; LOCK TABLE {} IN SHARE MODE",
        qualified_table(TableNamespace::Framework, "refresh_tokens")
    ))
    .await
    .expect("the gate holds the refresh tokens against insertions");

    let (answer, replayed) = tokio::join!(
        fixture.post_form("/token", &PORTAL_CREDENTIALS, &exchange),
        async {
            fixture.storage.administration.await_lock_waiters(1).await;

            let (replayed, ()) = tokio::join!(
                fixture.post_form("/token", &PORTAL_CREDENTIALS, &exchange),
                async {
                    fixture.storage.administration.await_lock_waiters(2).await;
                    gate.batch_execute("ROLLBACK")
                        .await
                        .expect("the gate opens");
                }
            );

            replayed
        }
    );
    let refreshed = fixture
        .post_form(
            "/token",
            &PORTAL_CREDENTIALS,
            &json!({
                "grant_type": "refresh_token",
                "refresh_token": answer.body["refresh_token"],
            }),
        )
        .await;

    assert_eq!(answer.status, 200);
    assert_eq!(replayed.status, 400);
    assert_eq!(replayed.body["error_description"], REPLAYED_CODE);
    assert_eq!(refreshed.body["error"], "invalid_grant");

    fixture.stop().await;
}
