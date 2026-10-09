use anyhow::Result;
use serde_json::json;

use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::published_key_set::published_key_set;
use margaret_jws_verification::accepted_key_set_document::AcceptedKeySetDocument;
use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_jws_verification::jws_verification::JwsVerification;
use margaret_jws_verification::key_set_document_parsing::KeySetDocumentParsing;

#[test]
fn signs_an_rs256_token_that_its_published_set_verifies() -> Result<()> {
    let secret = fresh_secret(SigningCurve::P256);
    let token = secret
        .rsa()
        .current()
        .sign_jwt(&json!({ "sub": "subject" }))?;
    let KeySetDocumentParsing::Accepted(AcceptedKeySetDocument { key_set, .. }) =
        published_key_set(&secret)
    else {
        panic!("the published key set is accepted");
    };
    let CompactJwsParsing::Parsed(jws) = CompactJws::parse(&token) else {
        panic!("the token parses");
    };

    assert!(matches!(key_set.verify(&jws), JwsVerification::Verified(_)));

    Ok(())
}
