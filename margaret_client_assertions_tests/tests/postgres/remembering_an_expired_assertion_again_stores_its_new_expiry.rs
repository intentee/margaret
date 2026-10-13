use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret_accepted_clients::client_assertion_max_lifetime::CLIENT_ASSERTION_MAX_LIFETIME;
use margaret_client_assertions::assertion_memory::AssertionMemory;
use margaret_client_assertions::client_assertion::ClientAssertion;
use margaret_client_assertions::remembered_assertion_key::RememberedAssertionKey;
use margaret_client_assertions_tests::contract_assertion_expiry::contract_assertion_expiry;
use margaret_client_assertions_tests::started_with_client_assertions::started_with_client_assertions;
use margaret_database_tests::contract_instant::contract_instant;
use margaret_database_tests::contract_token::contract_token;

#[tokio::test]
async fn remembering_an_expired_assertion_again_stores_its_new_expiry() {
    let started = started_with_client_assertions().await;
    let database = started.database.as_ref();
    let assertion = contract_token();
    let expires_at = contract_assertion_expiry();
    let extended = expires_at.after(CLIENT_ASSERTION_MAX_LIFETIME);

    assert_eq!(
        ClientAssertion::remember(
            database,
            "contract-client",
            assertion,
            expires_at,
            contract_instant()
        )
        .await
        .expect("the client assertion is remembered"),
        AssertionMemory::First
    );
    assert_eq!(
        ClientAssertion::remember(database, "contract-client", assertion, extended, expires_at)
            .await
            .expect("the expired client assertion is remembered again"),
        AssertionMemory::First
    );
    assert!(matches!(
        ClientAssertion::query()
            .key
            .eq(RememberedAssertionKey::of("contract-client", assertion)
                .as_bytes()
                .to_vec())
            .find(database)
            .await
            .expect("the remembered client assertion is found"),
        Lookup::Found(ClientAssertion { expires_at, .. }) if expires_at == extended.seconds_since_epoch()
    ));
}
