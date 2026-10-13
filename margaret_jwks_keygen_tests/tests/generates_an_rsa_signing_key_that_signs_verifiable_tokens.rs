use anyhow::Result;
use serde_json::json;

use margaret_jwks_keygen::generated_rsa_signing_keys::GeneratedRsaSigningKeys;
use margaret_jwks_keygen::provides_rsa_signing_keys::ProvidesRsaSigningKeys;
use margaret_jwks_keygen::rsa_jwk_pair::RsaJwkPair;
use margaret_jws_verification::accepted_key_set_document::AcceptedKeySetDocument;
use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_jws_verification::jws_verification::JwsVerification;
use margaret_jws_verification::key_id::KeyId;
use margaret_jws_verification::key_set_document_parsing::KeySetDocumentParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;

#[test]
fn generates_an_rsa_signing_key_that_signs_verifiable_tokens() -> Result<()> {
    let pair = RsaJwkPair::new(
        KeyId::new("generated".to_string()),
        GeneratedRsaSigningKeys.rsa_signing_key()?,
    )?;
    let token = pair.sign_jwt(&json!({ "sub": "subject" }))?;
    let KeySetDocumentParsing::Accepted(AcceptedKeySetDocument { key_set, .. }) =
        VerificationKeySet::parse(&serde_json::to_vec(
            &json!({ "keys": [pair.public_jwk()] }),
        )?)
    else {
        panic!("the published key is accepted");
    };
    let CompactJwsParsing::Parsed(jws) = CompactJws::parse(&token) else {
        panic!("the token parses");
    };

    assert!(matches!(key_set.verify(&jws), JwsVerification::Verified(_)));

    Ok(())
}
