use anyhow::Result;
use serde_json::from_str;

use margaret_jwks_keygen::jwk_public::JwkPublic;
use margaret_jwks_keygen::public_jwks::PublicJwks;
use margaret_jwks_keygen_tests::github_jwks_document::GITHUB_JWKS_DOCUMENT;

#[test]
fn public_jwks_reads_the_github_oidc_document() -> Result<()> {
    let public_jwks: PublicJwks = from_str(GITHUB_JWKS_DOCUMENT)?;

    assert!(!public_jwks.keys().is_empty());

    for key in public_jwks.keys() {
        assert!(matches!(key, JwkPublic::Rsa(_)));
        assert!(public_jwks.find_by_kid(key.kid()).is_some());
    }

    Ok(())
}
