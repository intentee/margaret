use std::ops::ControlFlow;
use std::time::Duration;

use margaret_jws_verification::ec_jwk::EcJwk;
use margaret_jws_verification::jwk::Jwk;
use margaret_jws_verification::public_jwk_material::public_jwk_material;
use margaret_jws_verification::rsa_jwk::RsaJwk;
use margaret_jws_verification::verification_key::VerificationKey;
use margaret_registered_claims::numeric_date::NumericDate;

use crate::jwks_key_error::JwksKeyError;

#[derive(Clone)]
pub struct RetiredKey {
    public_jwk: Jwk,
    retired_at: NumericDate,
    verification_key: VerificationKey,
}

impl RetiredKey {
    pub(crate) fn new(
        public_jwk: Jwk,
        verification_key: VerificationKey,
        retired_at: NumericDate,
    ) -> Self {
        Self {
            public_jwk,
            retired_at,
            verification_key,
        }
    }

    pub(crate) fn restore(public_jwk: Jwk, retired_at: NumericDate) -> Result<Self, JwksKeyError> {
        let (Jwk::Ec(EcJwk { kid: Some(kid), .. }) | Jwk::Rsa(RsaJwk { kid: Some(kid), .. })) =
            &public_jwk
        else {
            return Err(JwksKeyError::RetiredKeyWithoutKid);
        };

        match public_jwk_material(&public_jwk) {
            ControlFlow::Continue(material) => Ok(Self {
                verification_key: VerificationKey::new(kid.clone(), material),
                public_jwk,
                retired_at,
            }),
            ControlFlow::Break(reason) => Err(JwksKeyError::RetiredKeyRejected { reason }),
        }
    }

    #[must_use]
    pub fn public_jwk(&self) -> &Jwk {
        &self.public_jwk
    }

    #[must_use]
    pub fn retired_at(&self) -> NumericDate {
        self.retired_at
    }

    #[must_use]
    pub fn verification_key(&self) -> &VerificationKey {
        &self.verification_key
    }

    pub(crate) fn outlives(&self, window: Duration, moment: NumericDate) -> bool {
        self.retired_at.after(window) > moment
    }
}
