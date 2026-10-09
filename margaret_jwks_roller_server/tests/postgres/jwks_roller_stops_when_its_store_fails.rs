use std::sync::Arc;

use tokio_util::sync::CancellationToken;

use margaret_database_tests::table_privilege::TablePrivilege;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller::roller_error::RollerError;
use margaret_jwks_roller_server::jwks_roller::JwksRoller;
use margaret_jwks_roller_server::jwks_roller_server_error::JwksRollerServerError;
use margaret_signing_keys_tests::started_with_signing_keys::started_with_signing_keys;
use margaret_sql_identifier::table_namespace::TableNamespace;

#[tokio::test(start_paused = true)]
async fn jwks_roller_stops_when_its_store_fails() {
    let started = started_with_signing_keys().await;
    let roller = JwksRoller::create(
        Arc::clone(&started.database),
        Arc::new(FixtureRsaSigningKeys::default()),
    )
    .await
    .expect("the roller starts");

    started
        .administration
        .revoke(
            TablePrivilege::Select,
            TableNamespace::Framework,
            "signing_key_sets",
        )
        .await;

    assert!(matches!(
        roller.run(CancellationToken::new()).await,
        Err(JwksRollerServerError::SecretRoll(
            RollerError::SecretLoad { .. }
        ))
    ));
}
