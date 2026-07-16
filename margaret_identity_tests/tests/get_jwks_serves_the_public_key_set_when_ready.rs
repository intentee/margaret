use std::sync::Arc;

use margaret_identity::routes::public::get_jwks::GetJwks;
use margaret_identity::stores::signing_key_store::SigningKeyStore;
use margaret_jwks_key_gen::curve::Curve;
use margaret_jwks_key_gen::jwks_secret::JwksSecret;

#[tokio::test]
async fn get_jwks_serves_the_public_key_set_when_ready() {
    let store = Arc::new(SigningKeyStore::create());
    let secret = JwksSecret::fresh(Curve::P256).expect("a fresh secret is generated");
    store.holder().set(Some(Arc::new(secret)));
    let route = GetJwks::create(store);

    let response = route.respond().await;

    assert_eq!(response.status(), 200);
}
