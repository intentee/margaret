use serde_json::Value;

use margaret_jose_parameters::curve::Curve;
use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_key::FixtureKey;

pub struct SignedClaims {
    pub key_set: KeySetParsing,
    pub token: String,
}

impl SignedClaims {
    #[must_use]
    pub fn new(claims: &Value) -> Self {
        let key = FixtureKey::generate(Curve::P256, "kid");

        Self {
            key_set: VerificationKeySet::from_jwks(vec![key.jwk()]),
            token: key.token(&key.header(), claims),
        }
    }
}
