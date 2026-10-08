use std::sync::Arc;

use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;

#[test]
fn jwks_secret_holder_returns_set_secret() {
    let secret = Arc::new(fresh_secret(SigningCurve::P256));
    let initial = Arc::new(fresh_secret(SigningCurve::P256));
    let holder = JwksSecretHolder::new(Arc::clone(&initial));

    assert!(Arc::ptr_eq(&holder.get(), &initial));

    holder.set(Arc::clone(&secret));

    assert!(Arc::ptr_eq(&holder.get(), &secret));

    let cloned_holder = holder.clone();

    assert!(Arc::ptr_eq(&cloned_holder.get(), &secret));
}
