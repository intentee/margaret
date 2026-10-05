use aws_lc_rs::error::Unspecified;

use margaret_jwks_keygen::jwks_key_error::JwksKeyError;
use margaret_jwks_secret_store::jwks_secret_store_error::JwksSecretStoreError;
use margaret_registered_claims::claims_merge_error::ClaimsMergeError;

#[test]
fn describes_every_store_error() {
    assert_eq!(
        JwksSecretStoreError::AccessTokenClaims(ClaimsMergeError::NotAnObject).to_string(),
        "the access token claims could not be merged: the application claims are not a json object"
    );
    assert_eq!(
        JwksSecretStoreError::IdTokenSigning(JwksKeyError::RsaSigning {
            source: Unspecified
        })
        .to_string(),
        format!(
            "the id token could not be signed with the rsa key: the rsa signing key could not sign: {Unspecified}"
        )
    );
}
