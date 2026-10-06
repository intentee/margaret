use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::ops::ControlFlow;

use serde_json::Value;

use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;

use crate::accepted_key_set_document::AcceptedKeySetDocument;
use crate::admitted_key::AdmittedKey;
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
use crate::verification_material::VerificationMaterial;
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
    identified: HashMap<KeyId, VerificationMaterial>,
    unidentified: Vec<VerificationMaterial>,
}

impl VerificationKeySet {
    #[must_use]
    pub fn assemble(keys: Vec<VerificationKey>) -> KeySetAssembly {
        let mut identified = HashMap::with_capacity(keys.len());

        for VerificationKey { kid, material } in keys {
            match identified.entry(kid) {
                Entry::Occupied(occupied) => {
                    return KeySetAssembly::DuplicateKeyId(DuplicateKeyId {
                        kid: occupied.key().clone(),
                    });
                }
                Entry::Vacant(vacant) => {
                    vacant.insert(material);
                }
            }
        }

        KeySetAssembly::Assembled(Self {
            identified,
            unidentified: Vec::new(),
        })
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
        let mut identified_keys = Vec::with_capacity(published.len());
        let mut unidentified = Vec::new();
        let mut disclosed_keys = Vec::new();
        let mut ignored_keys = Vec::new();

        for (index, entry) in published.into_iter().enumerate() {
            let admission = match entry {
                ControlFlow::Continue(published) => published.into_verification_key(composition),
                ControlFlow::Break(reason) => ControlFlow::Break(KeyExclusion::Ignored(reason)),
            };

            match admission {
                ControlFlow::Continue(AdmittedKey::Identified(key)) => identified_keys.push(key),
                ControlFlow::Continue(AdmittedKey::Unidentified(material)) => {
                    unidentified.push(material);
                }
                ControlFlow::Break(KeyExclusion::Disclosed(disclosure)) => {
                    disclosed_keys.push(DisclosedKey { disclosure, index });
                }
                ControlFlow::Break(KeyExclusion::Ignored(reason)) => {
                    ignored_keys.push(IgnoredKey { index, reason });
                }
            }
        }

        match Self::assemble(identified_keys) {
            KeySetAssembly::Assembled(Self { identified, .. }) => {
                KeySetDocumentParsing::Accepted(AcceptedKeySetDocument {
                    disclosed_keys,
                    ignored_keys,
                    key_set: Self {
                        identified,
                        unidentified,
                    },
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
        let material = self.material_for(kid.as_ref(), algorithm)?;

        match material.check(signing_input.as_bytes(), signature) {
            SignatureCheck::LengthMismatch { expected, found } => {
                ControlFlow::Break(JwsRejection::SignatureLength { expected, found })
            }
            SignatureCheck::Matches => ControlFlow::Continue(VerifiedJws { kid: kid.as_ref() }),
            SignatureCheck::Mismatch(source) => {
                ControlFlow::Break(JwsRejection::SignatureMismatch { algorithm, source })
            }
        }
    }

    fn material_for(
        &self,
        kid: Option<&KeyId>,
        algorithm: JwsAlgorithm,
    ) -> ControlFlow<JwsRejection, &VerificationMaterial> {
        match kid {
            Some(kid) => match self.identified.get(kid) {
                Some(material) if material.admits(algorithm) => ControlFlow::Continue(material),
                Some(material) => ControlFlow::Break(JwsRejection::AlgorithmMismatch {
                    key: material.algorithm(),
                    token: algorithm,
                }),
                None => ControlFlow::Break(JwsRejection::UnknownKeyId { kid: kid.clone() }),
            },
            None => match self
                .identified
                .values()
                .chain(&self.unidentified)
                .filter(|material| material.admits(algorithm))
                .collect::<Vec<&VerificationMaterial>>()
                .as_slice()
            {
                [material] => ControlFlow::Continue(*material),
                candidates => ControlFlow::Break(JwsRejection::MissingKeyId {
                    candidates: candidates.len(),
                }),
            },
        }
    }
}
