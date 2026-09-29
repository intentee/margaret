use serde_json::Value;
use serde_json::json;

use margaret_jose_parameters::curve::Curve;
use margaret_jws_verification::key_set_assembly::KeySetAssembly;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_key::FixtureKey;

pub struct SignedClaims {
    pub key_set: KeySetAssembly,
    pub token: String,
}

impl SignedClaims {
    #[must_use]
    pub fn new(claims: &Value) -> Self {
        let key = FixtureKey::generate(Curve::P256, "kid");

        Self::signed(&key, &key.header(), claims)
    }

    #[must_use]
    pub fn typed(typ: &str, claims: &Value) -> Self {
        let key = FixtureKey::generate(Curve::P256, "kid");
        let mut header = key.header();

        header["typ"] = json!(typ);

        Self::signed(&key, &header, claims)
    }

    fn signed(key: &FixtureKey, header: &Value, claims: &Value) -> Self {
        Self {
            key_set: VerificationKeySet::assemble(vec![key.verification_key()]),
            token: key.token(header, claims),
        }
    }
}
