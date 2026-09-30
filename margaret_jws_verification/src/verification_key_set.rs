use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::ops::ControlFlow;

use serde_json::Value;

use crate::accepted_key_set_document::AcceptedKeySetDocument;
use crate::compact_jws::CompactJws;
use crate::disclosed_key::DisclosedKey;
use crate::duplicate_key_id::DuplicateKeyId;
use crate::ignored_key::IgnoredKey;
use crate::ignored_key_reason::IgnoredKeyReason;
use crate::jws_header::JwsHeader;
use crate::jws_rejection::JwsRejection;
use crate::jws_verification::JwsVerification;
use crate::key_exclusion::KeyExclusion;
use crate::key_id::KeyId;
use crate::key_set_assembly::KeySetAssembly;
use crate::key_set_composition::KeySetComposition;
use crate::key_set_document::KeySetDocument;
use crate::key_set_document_parsing::KeySetDocumentParsing;
use crate::key_set_document_rejection::KeySetDocumentRejection;
use crate::parameter_value::ParameterValue;
use crate::published_jwk::PublishedJwk;
use crate::signature_check::SignatureCheck;
use crate::verification_key::VerificationKey;
use crate::verified_jws::VerifiedJws;

fn published_jwk(entry: Value) -> ControlFlow<IgnoredKeyReason, PublishedJwk> {
    match serde_json::from_value(entry) {
        Ok(published) => ControlFlow::Continue(published),
        Err(source) => ControlFlow::Break(IgnoredKeyReason::Malformed { source }),
    }
}

fn composition(published: &[ControlFlow<IgnoredKeyReason, PublishedJwk>]) -> KeySetComposition {
    if published.iter().any(|entry| {
        matches!(entry, ControlFlow::Continue(published) if published.declares_encryption())
    }) {
        KeySetComposition::IncludesEncryptionKeys
    } else {
        KeySetComposition::SignatureKeysOnly
    }
}

#[derive(Clone)]
pub struct VerificationKeySet {
    keys: HashMap<KeyId, VerificationKey>,
}

impl VerificationKeySet {
    #[must_use]
    pub fn assemble(verification_keys: Vec<VerificationKey>) -> KeySetAssembly {
        let mut keys = HashMap::with_capacity(verification_keys.len());

        for key in verification_keys {
            match keys.entry(key.kid.clone()) {
                Entry::Occupied(occupied) => {
                    return KeySetAssembly::DuplicateKeyId(DuplicateKeyId {
                        kid: occupied.key().clone(),
                    });
                }
                Entry::Vacant(vacant) => {
                    vacant.insert(key);
                }
            }
        }

        KeySetAssembly::Assembled(Self { keys })
    }

    #[must_use]
    pub fn parse(document: &[u8]) -> KeySetDocumentParsing {
        let KeySetDocument { keys } = match serde_json::from_slice(document) {
            Ok(document) => document,
            Err(source) => {
                return KeySetDocumentParsing::Rejected(KeySetDocumentRejection::Malformed {
                    source,
                });
            }
        };
        let published = keys.into_iter().map(published_jwk).collect::<Vec<_>>();
        let composition = composition(&published);
        let mut admitted_keys = Vec::with_capacity(published.len());
        let mut disclosed_keys = Vec::new();
        let mut ignored_keys = Vec::new();

        for (index, entry) in published.into_iter().enumerate() {
            let admission = match entry {
                ControlFlow::Continue(published) => published.into_verification_key(composition),
                ControlFlow::Break(reason) => ControlFlow::Break(KeyExclusion::Ignored(reason)),
            };

            match admission {
                ControlFlow::Continue(key) => admitted_keys.push(key),
                ControlFlow::Break(KeyExclusion::Disclosed(disclosure)) => {
                    disclosed_keys.push(DisclosedKey { disclosure, index });
                }
                ControlFlow::Break(KeyExclusion::Ignored(reason)) => {
                    ignored_keys.push(IgnoredKey { index, reason });
                }
            }
        }

        match Self::assemble(admitted_keys) {
            KeySetAssembly::Assembled(key_set) => {
                KeySetDocumentParsing::Accepted(AcceptedKeySetDocument {
                    disclosed_keys,
                    ignored_keys,
                    key_set,
                })
            }
            KeySetAssembly::DuplicateKeyId(duplicate) => {
                KeySetDocumentParsing::Rejected(KeySetDocumentRejection::DuplicateKeyId(duplicate))
            }
        }
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
            header: JwsHeader { alg, crit, kid, .. },
            signature,
            signing_input,
            ..
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

        if !key.material.admits(algorithm) {
            return ControlFlow::Break(JwsRejection::AlgorithmMismatch {
                key: key.material.algorithm(),
                token: algorithm,
            });
        }

        match key.material.check(signing_input.as_bytes(), signature) {
            SignatureCheck::LengthMismatch { expected, found } => {
                ControlFlow::Break(JwsRejection::SignatureLength { expected, found })
            }
            SignatureCheck::Matches => ControlFlow::Continue(VerifiedJws { kid }),
            SignatureCheck::Mismatch(source) => {
                ControlFlow::Break(JwsRejection::SignatureMismatch { algorithm, source })
            }
        }
    }
}
