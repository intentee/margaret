use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use zeroize::Zeroizing;

use crate::curve::Curve;
use crate::jwk_pair::JwkPair;
use crate::jwk_public::JwkPublic;
use crate::jwk_signing::JwkSigning;
use crate::jwks_secret::JwksSecret;

pub struct PersistedJwksSecret {
    secret: JwksSecret,
}

impl PersistedJwksSecret {
    #[must_use]
    pub fn new(secret: JwksSecret) -> Self {
        Self { secret }
    }

    #[must_use]
    pub fn into_secret(self) -> JwksSecret {
        self.secret
    }
}

impl Serialize for PersistedJwksSecret {
    fn serialize<Target>(&self, serializer: Target) -> Result<Target::Ok, Target::Error>
    where
        Target: Serializer,
    {
        #[derive(Serialize)]
        struct SigningWire<'material> {
            crv: Curve,
            kid: &'material str,
            pem: &'material str,
        }

        #[derive(Serialize)]
        struct PairWire<'material> {
            public: &'material JwkPublic,
            signing: SigningWire<'material>,
        }

        #[derive(Serialize)]
        struct SecretWire<'material> {
            current: PairWire<'material>,
            next: PairWire<'material>,
            previous: PairWire<'material>,
        }

        fn borrow(pair: &JwkPair) -> PairWire<'_> {
            PairWire {
                public: &pair.public,
                signing: SigningWire {
                    crv: pair.signing.crv,
                    kid: pair.signing.kid.as_str(),
                    pem: pair.signing.pem.as_str(),
                },
            }
        }

        SecretWire {
            current: borrow(&self.secret.current),
            next: borrow(&self.secret.next),
            previous: borrow(&self.secret.previous),
        }
        .serialize(serializer)
    }
}

impl<'wire> Deserialize<'wire> for PersistedJwksSecret {
    fn deserialize<Source>(deserializer: Source) -> Result<Self, Source::Error>
    where
        Source: Deserializer<'wire>,
    {
        #[derive(Deserialize)]
        struct SigningWire {
            crv: Curve,
            kid: String,
            pem: String,
        }

        #[derive(Deserialize)]
        struct PairWire {
            public: JwkPublic,
            signing: SigningWire,
        }

        #[derive(Deserialize)]
        struct SecretWire {
            current: PairWire,
            next: PairWire,
            previous: PairWire,
        }

        fn restore(PairWire { public, signing }: PairWire) -> JwkPair {
            JwkPair {
                public,
                signing: JwkSigning {
                    crv: signing.crv,
                    kid: signing.kid,
                    pem: Zeroizing::new(signing.pem),
                },
            }
        }

        let SecretWire {
            current,
            next,
            previous,
        } = SecretWire::deserialize(deserializer)?;

        Ok(Self {
            secret: JwksSecret {
                current: restore(current),
                next: restore(next),
                previous: restore(previous),
            },
        })
    }
}
