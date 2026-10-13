use std::sync::Arc;

use bytes::Bytes;
use tokio_util::sync::CancellationToken;

use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller::roll_due_at::roll_due_at;
use margaret_jwks_roller_server::jwks_roller::JwksRoller;
use margaret_jwks_roller_tests::fixture_synchronizer::fixture_synchronizer;
use margaret_signing_keys::signing_keys_revision::SigningKeysRevision;
use margaret_signing_keys_tests::started_with_signing_keys::started_with_signing_keys;
use margaret_signing_keys_tests::stored_revision::StoredRevision;

#[tokio::test(start_paused = true)]
async fn jwks_roller_adopts_the_roll_of_a_peer_once_its_keys_are_due() {
    let started = started_with_signing_keys().await;
    let roller = Arc::new(
        JwksRoller::create(
            Arc::clone(&started.database),
            Arc::new(FixtureRsaSigningKeys::default()),
        )
        .await
        .expect("the roller starts"),
    );
    let first = roller.jwks_secret_holder().get();
    let peer_rolled = fixture_synchronizer(started.separate_pool().await)
        .synchronized(&HeldSecret::Held(first.clone()), roll_due_at(&first))
        .await
        .expect("the peer rolls the keys");
    let mut secret_subscription = roller.jwks_secret_holder().subscribe();
    let cancellation_token = CancellationToken::new();
    let running = tokio::spawn({
        let roller = roller.clone();
        let cancellation_token = cancellation_token.clone();

        async move { roller.run(cancellation_token).await }
    });

    secret_subscription.changed().await;
    cancellation_token.cancel();
    running
        .await
        .expect("the roller joins")
        .expect("the roller stops cleanly");

    assert_eq!(
        secret_subscription.read_current().generation(),
        peer_rolled.generation()
    );
    assert_eq!(
        roller.jwks_document_holder().get(),
        Bytes::from(
            serde_json::to_vec(peer_rolled.public_jwks()).expect("the public key set serializes")
        )
    );
    assert_eq!(
        StoredRevision::loaded(&started.database).await,
        StoredRevision::of(
            &SigningKeysRevision::from_secret(&peer_rolled).expect("the keys serialize")
        )
    );
}
