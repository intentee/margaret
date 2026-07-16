use std::sync::Arc;

use margaret_identity::stores::signing_key_store::SigningKeyStore;
use margaret_identity::routes::public::get_jwks::GetJwks;

#[tokio::test]
async fn get_jwks_returns_503_when_the_keys_are_not_ready() {
    let store = Arc::new(SigningKeyStore::create());
    let route = GetJwks::create(store);

    let response = route.respond().await;

    assert_eq!(response.status(), 503);
}
