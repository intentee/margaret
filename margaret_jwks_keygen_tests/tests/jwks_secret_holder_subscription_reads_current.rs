use std::sync::Arc;

use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;

#[test]
fn jwks_secret_holder_subscription_reads_current() {
    let secret = Arc::new(fresh_secret(SigningCurve::P256));
    let holder = JwksSecretHolder::new(Arc::clone(&secret));
    let mut subscription = holder.subscribe();

    assert!(Arc::ptr_eq(&subscription.read_current(), &secret));
}
