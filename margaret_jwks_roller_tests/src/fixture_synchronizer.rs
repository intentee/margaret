use std::sync::Arc;

use margaret_database::database::Database;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller::signing_keys_synchronizer::SigningKeysSynchronizer;

#[must_use]
pub fn fixture_synchronizer(database: Arc<Database>) -> SigningKeysSynchronizer {
    SigningKeysSynchronizer {
        curve: SigningCurve::P256,
        database,
        rsa_keys: Arc::new(FixtureRsaSigningKeys::default()),
    }
}
