use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::ops::ControlFlow;

use serde::Deserialize;
use serde_json::Value;

use crate::compact_jws::CompactJws;
use crate::header_algorithm::HeaderAlgorithm;
use crate::jwk::Jwk;
use crate::jwk_rejection::JwkRejection;
use crate::jws_header::JwsHeader;
use crate::jws_rejection::JwsRejection;
use crate::jws_verification::JwsVerification;
use crate::key_id::KeyId;
use crate::key_set_parsing::KeySetParsing;
use crate::key_set_rejection::KeySetRejection;
use crate::signature_check::SignatureCheck;
use crate::verification_key::VerificationKey;
use crate::verified_jws::VerifiedJws;

#[derive(Deserialize)]
struct KeySetDocument {
    keys: Vec<Value>,
}

#[derive(Clone)]
pub struct VerificationKeySet {
    keys: HashMap<KeyId, VerificationKey>,
}

impl VerificationKeySet {
    #[must_use]
    pub fn from_jwks(jwks: Vec<Jwk>) -> KeySetParsing {
        let mut keys = HashMap::with_capacity(jwks.len());

        for (index, jwk) in jwks.into_iter().enumerate() {
            let key = match VerificationKey::from_jwk(jwk) {
                ControlFlow::Continue(key) => key,
                ControlFlow::Break(rejection) => {
                    return KeySetParsing::Rejected(KeySetRejection::Key { index, rejection });
                }
            };

            match keys.entry(key.kid.clone()) {
                Entry::Occupied(occupied) => {
                    return KeySetParsing::Rejected(KeySetRejection::DuplicateKeyId {
                        kid: occupied.key().clone(),
                    });
                }
                Entry::Vacant(vacant) => {
                    vacant.insert(key);
                }
            }
        }

        KeySetParsing::Accepted(Self { keys })
    }

    #[must_use]
    pub fn parse(document: &[u8]) -> KeySetParsing {
        let KeySetDocument { keys } = match serde_json::from_slice(document) {
            Ok(document) => document,
            Err(source) => return KeySetParsing::Rejected(KeySetRejection::Malformed { source }),
        };
        let mut jwks = Vec::with_capacity(keys.len());

        for (index, key) in keys.into_iter().enumerate() {
            match serde_json::from_value(key) {
                Ok(jwk) => jwks.push(jwk),
                Err(source) => {
                    return KeySetParsing::Rejected(KeySetRejection::Key {
                        index,
                        rejection: JwkRejection::Malformed { source },
                    });
                }
            }
        }

        Self::from_jwks(jwks)
    }

    #[must_use]
    pub fn verify<'jws>(&self, jws: &'jws CompactJws<'_>) -> JwsVerification<'jws> {
        match self.verified(jws) {
            ControlFlow::Continue(verified) => JwsVerification::Verified(verified),
            ControlFlow::Break(rejection) => JwsVerification::Rejected(rejection),
        }
    }

    fn verified<'jws>(
        &self,
        CompactJws {
            header:
                JwsHeader {
                    alg,
                    crit,
                    kid,
                    typ,
                },
            payload,
            signature,
            signing_input,
        }: &'jws CompactJws<'_>,
    ) -> ControlFlow<JwsRejection, VerifiedJws<'jws>> {
        if crit.is_some() {
            return ControlFlow::Break(JwsRejection::CriticalHeader);
        }

        let algorithm = match alg {
            HeaderAlgorithm::Supported(algorithm) => *algorithm,
            HeaderAlgorithm::Unsupported(alg) => {
                return ControlFlow::Break(JwsRejection::UnsupportedAlgorithm { alg: alg.clone() });
            }
        };
        let Some(kid) = kid else {
            return ControlFlow::Break(JwsRejection::MissingKeyId);
        };
        let Some(key) = self.keys.get(kid) else {
            return ControlFlow::Break(JwsRejection::UnknownKeyId { kid: kid.clone() });
        };

        if key.algorithm != algorithm {
            return ControlFlow::Break(JwsRejection::AlgorithmMismatch {
                key: key.algorithm,
                token: algorithm,
            });
        }

        match key.material.check(signing_input.as_bytes(), signature) {
            SignatureCheck::EcdsaMalformed(source) => {
                ControlFlow::Break(JwsRejection::EcdsaSignatureMalformed { source })
            }
            SignatureCheck::EcdsaMismatch(source) => {
                ControlFlow::Break(JwsRejection::EcdsaSignatureMismatch { source })
            }
            SignatureCheck::Matches => ControlFlow::Continue(VerifiedJws {
                kid,
                payload,
                typ: typ.as_ref(),
            }),
            SignatureCheck::RsaMismatch(source) => {
                ControlFlow::Break(JwsRejection::RsaSignatureMismatch { source })
            }
        }
    }
}
