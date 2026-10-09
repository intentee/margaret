use std::sync::Arc;

use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller::signing_keys_synchronizer::SigningKeysSynchronizer;
use margaret_jwks_roller::stores_signing_keys::StoresSigningKeys;

#[must_use]
pub fn fixture_synchronizer(storage: Arc<dyn StoresSigningKeys>) -> SigningKeysSynchronizer {
    SigningKeysSynchronizer {
        curve: SigningCurve::P256,
        rsa_keys: Arc::new(FixtureRsaSigningKeys::default()),
        storage,
    }
}
