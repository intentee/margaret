use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;
use ring::signature::RSA_PKCS1_2048_8192_SHA256;
use ring::signature::RsaPublicKeyComponents;

use crate::jwks_key_error::JwksKeyError;
use crate::signature_check::SignatureCheck;

const RSA_PKCS1_2048_8192_SHA256_MINIMUM_MODULUS_BYTES: usize = 2048 / 8;

pub(crate) fn verify_rsa_signature(
    signing_input: &str,
    signature_bytes: &[u8],
    modulus: &str,
    exponent: &str,
) -> Result<SignatureCheck, JwksKeyError> {
    let n = Base64UrlUnpadded::decode_vec(modulus)
        .map_err(|source| JwksKeyError::RsaModulusBase64 { source })?;
    let e = Base64UrlUnpadded::decode_vec(exponent)
        .map_err(|source| JwksKeyError::RsaExponentBase64 { source })?;

    if n.len() < RSA_PKCS1_2048_8192_SHA256_MINIMUM_MODULUS_BYTES {
        return Err(JwksKeyError::RsaModulusTooShort {
            expected: RSA_PKCS1_2048_8192_SHA256_MINIMUM_MODULUS_BYTES,
            found: n.len(),
        });
    }

    Ok(SignatureCheck::from_verification(
        RsaPublicKeyComponents { n, e }
            .verify(
                &RSA_PKCS1_2048_8192_SHA256,
                signing_input.as_bytes(),
                signature_bytes,
            )
            .is_ok(),
    ))
}
