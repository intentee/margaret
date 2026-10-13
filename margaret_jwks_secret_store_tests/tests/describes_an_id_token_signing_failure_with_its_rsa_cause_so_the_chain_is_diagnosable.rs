use aws_lc_rs::error::Unspecified;

use margaret_jwks_keygen::jwks_key_error::JwksKeyError;
use margaret_jwks_secret_store::jwks_secret_store_error::JwksSecretStoreError;

#[test]
fn describes_an_id_token_signing_failure_with_its_rsa_cause_so_the_chain_is_diagnosable() {
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
