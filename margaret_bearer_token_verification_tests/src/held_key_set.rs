use std::sync::Arc;

use margaret_issuer_key_set::issuer_key_set::IssuerKeySet;
use margaret_jws_verification::verification_key_set::VerificationKeySet;

#[must_use]
pub fn held_key_set(key_set: VerificationKeySet) -> Arc<IssuerKeySet> {
    let issuer_key_set = Arc::new(IssuerKeySet::awaiting());

    issuer_key_set.start_fetch();
    issuer_key_set.hold(Arc::new(key_set));

    issuer_key_set
}
