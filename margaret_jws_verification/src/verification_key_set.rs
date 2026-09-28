use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::ops::ControlFlow;

use serde::Deserialize;
use serde_json::Value;

use crate::accepted_key_set_document::AcceptedKeySetDocument;
use crate::compact_jws::CompactJws;
use crate::ignored_key::IgnoredKey;
use crate::ignored_key_reason::IgnoredKeyReason;
use crate::jwk::Jwk;
use crate::jws_header::JwsHeader;
use crate::jws_rejection::JwsRejection;
use crate::jws_verification::JwsVerification;
use crate::key_id::KeyId;
use crate::key_set_document_parsing::KeySetDocumentParsing;
use crate::key_set_document_rejection::KeySetDocumentRejection;
use crate::key_set_parsing::KeySetParsing;
use crate::key_set_rejection::KeySetRejection;
use crate::parameter_value::ParameterValue;
use crate::published_jwk::PublishedJwk;
use crate::signature_check::SignatureCheck;
use crate::verification_key::VerificationKey;
use crate::verified_jws::VerifiedJws;

#[derive(Deserialize)]
struct KeySetDocument {
    keys: Vec<Value>,
}

fn insert_unique(
    keys: &mut HashMap<KeyId, VerificationKey>,
    key: VerificationKey,
) -> ControlFlow<KeyId> {
    match keys.entry(key.kid.clone()) {
        Entry::Occupied(occupied) => ControlFlow::Break(occupied.key().clone()),
        Entry::Vacant(vacant) => {
            vacant.insert(key);

            ControlFlow::Continue(())
        }
    }
}

fn usable_key(published: Value) -> ControlFlow<IgnoredKeyReason, VerificationKey> {
    let published: PublishedJwk = match serde_json::from_value(published) {
        Ok(published) => published,
        Err(source) => return ControlFlow::Break(IgnoredKeyReason::Malformed { source }),
    };

    match VerificationKey::from_jwk(published.into_jwk()?) {
        ControlFlow::Continue(key) => ControlFlow::Continue(key),
        ControlFlow::Break(rejection) => {
            ControlFlow::Break(IgnoredKeyReason::Unusable { rejection })
        }
    }
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

            if let ControlFlow::Break(kid) = insert_unique(&mut keys, key) {
                return KeySetParsing::Rejected(KeySetRejection::DuplicateKeyId { kid });
            }
        }

        KeySetParsing::Accepted(Self { keys })
    }

    #[must_use]
    pub fn parse(document: &[u8]) -> KeySetDocumentParsing {
        let KeySetDocument {
            keys: published_keys,
        } = match serde_json::from_slice(document) {
            Ok(document) => document,
            Err(source) => {
                return KeySetDocumentParsing::Rejected(KeySetDocumentRejection::Malformed {
                    source,
                });
            }
        };
        let mut keys = HashMap::with_capacity(published_keys.len());
        let mut ignored_keys = Vec::new();

        for (index, published) in published_keys.into_iter().enumerate() {
            match usable_key(published) {
                ControlFlow::Continue(key) => {
                    if let ControlFlow::Break(kid) = insert_unique(&mut keys, key) {
                        return KeySetDocumentParsing::Rejected(
                            KeySetDocumentRejection::DuplicateKeyId { kid },
                        );
                    }
                }
                ControlFlow::Break(reason) => ignored_keys.push(IgnoredKey { index, reason }),
            }
        }

        KeySetDocumentParsing::Accepted(AcceptedKeySetDocument {
            ignored_keys,
            key_set: Self { keys },
        })
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
            ParameterValue::Supported(algorithm) => *algorithm,
            ParameterValue::Unsupported(alg) => {
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
