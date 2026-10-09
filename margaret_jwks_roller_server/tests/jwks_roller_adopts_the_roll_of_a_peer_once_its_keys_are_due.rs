use std::sync::Arc;

use bytes::Bytes;
use tokio_util::sync::CancellationToken;

use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller::roll_due_at::roll_due_at;
use margaret_jwks_roller_server::jwks_roller::JwksRoller;
use margaret_jwks_roller_tests::fixture_signing_keys::FixtureSigningKeys;
use margaret_jwks_roller_tests::fixture_synchronizer::fixture_synchronizer;

#[tokio::test(start_paused = true)]
async fn jwks_roller_adopts_the_roll_of_a_peer_once_its_keys_are_due() {
    let storage = Arc::new(FixtureSigningKeys::empty());
    let roller = Arc::new(
        JwksRoller::create(storage.clone(), Arc::new(FixtureRsaSigningKeys::default()))
            .await
            .expect("the roller starts"),
    );
    let started = roller.jwks_secret_holder().get();
    let peer_rolled = fixture_synchronizer(storage.clone())
        .synchronized(&HeldSecret::Held(started.clone()), roll_due_at(&started))
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
    assert_eq!(storage.accepted_writes().await, 2);
}
