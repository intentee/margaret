use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;
use p256::ecdsa::signature::Signer;
use p256::pkcs8::DecodePrivateKey;

use crate::curve::Curve;
use crate::jwks_key_error::JwksKeyError;

pub(crate) fn sign_signing_input(
    signing_input: &str,
    pem: &str,
    crv: Curve,
) -> Result<String, JwksKeyError> {
    let signature_bytes = match crv {
        Curve::P256 => {
            let signing_key = p256::ecdsa::SigningKey::from_pkcs8_pem(pem)
                .map_err(|source| JwksKeyError::SigningKeyRejected { source })?;
            let signature: p256::ecdsa::Signature = signing_key.sign(signing_input.as_bytes());

            signature
                .normalize_s()
                .unwrap_or(signature)
                .to_bytes()
                .to_vec()
        }
        Curve::P384 => {
            let signing_key = p384::ecdsa::SigningKey::from_pkcs8_pem(pem)
                .map_err(|source| JwksKeyError::SigningKeyRejected { source })?;
            let signature: p384::ecdsa::Signature = signing_key.sign(signing_input.as_bytes());

            signature
                .normalize_s()
                .unwrap_or(signature)
                .to_bytes()
                .to_vec()
        }
    };

    Ok(Base64UrlUnpadded::encode_string(&signature_bytes))
}

#[cfg(test)]
mod tests {
    use p256::pkcs8::DecodePrivateKey;

    use super::sign_signing_input;
    use crate::curve::Curve;

    #[test]
    fn p256_rejects_a_malformed_pem() {
        assert_eq!(
            sign_signing_input("header.payload", "not a pem", Curve::P256)
                .err()
                .unwrap()
                .to_string(),
            format!(
                "the signing key pem could not be parsed into an ecdsa signing key: {}",
                p256::ecdsa::SigningKey::from_pkcs8_pem("not a pem")
                    .err()
                    .unwrap()
            )
        );
    }

    #[test]
    fn p384_rejects_a_malformed_pem() {
        assert_eq!(
            sign_signing_input("header.payload", "not a pem", Curve::P384)
                .err()
                .unwrap()
                .to_string(),
            format!(
                "the signing key pem could not be parsed into an ecdsa signing key: {}",
                p384::ecdsa::SigningKey::from_pkcs8_pem("not a pem")
                    .err()
                    .unwrap()
            )
        );
    }
}
