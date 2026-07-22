use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;
use p256::ecdsa::signature::Verifier;

use crate::curve::Curve;
use crate::jwks_key_error::JwksKeyError;

fn sec1_point(x: &str, y: &str) -> Result<Vec<u8>, JwksKeyError> {
    let mut sec1 = vec![0x04u8];

    sec1.extend_from_slice(
        &Base64UrlUnpadded::decode_vec(x)
            .map_err(|source| JwksKeyError::CoordinateBase64 { source })?,
    );
    sec1.extend_from_slice(
        &Base64UrlUnpadded::decode_vec(y)
            .map_err(|source| JwksKeyError::CoordinateBase64 { source })?,
    );

    Ok(sec1)
}

pub(crate) fn verify_signature(
    signing_input: &str,
    signature_bytes: &[u8],
    x: &str,
    y: &str,
    crv: Curve,
) -> Result<bool, JwksKeyError> {
    let sec1 = sec1_point(x, y)?;

    match crv {
        Curve::P256 => {
            let verifying_key = p256::ecdsa::VerifyingKey::from_sec1_bytes(&sec1)
                .map_err(|source| JwksKeyError::MalformedVerifyingKey { source })?;
            let signature = p256::ecdsa::Signature::from_slice(signature_bytes)
                .map_err(|source| JwksKeyError::SignatureMalformed { source })?;

            if signature.normalize_s().is_some() {
                return Err(JwksKeyError::NonCanonicalSignature);
            }

            Ok(verifying_key
                .verify(signing_input.as_bytes(), &signature)
                .is_ok())
        }
        Curve::P384 => {
            let verifying_key = p384::ecdsa::VerifyingKey::from_sec1_bytes(&sec1)
                .map_err(|source| JwksKeyError::MalformedVerifyingKey { source })?;
            let signature = p384::ecdsa::Signature::from_slice(signature_bytes)
                .map_err(|source| JwksKeyError::SignatureMalformed { source })?;

            if signature.normalize_s().is_some() {
                return Err(JwksKeyError::NonCanonicalSignature);
            }

            Ok(verifying_key
                .verify(signing_input.as_bytes(), &signature)
                .is_ok())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use base64ct::Base64UrlUnpadded;
    use base64ct::Encoding;

    use super::verify_signature;
    use crate::curve::Curve;
    use crate::generate_keypair::generate_keypair;
    use crate::generate_keypair_params::GenerateKeypairParams;
    use crate::jwk_pair::JwkPair;

    static P256_KEYPAIR: LazyLock<JwkPair> = LazyLock::new(|| {
        generate_keypair(GenerateKeypairParams {
            crv: Curve::P256,
            kid: "p256".to_string(),
        })
        .unwrap()
    });
    static P384_KEYPAIR: LazyLock<JwkPair> = LazyLock::new(|| {
        generate_keypair(GenerateKeypairParams {
            crv: Curve::P384,
            kid: "p384".to_string(),
        })
        .unwrap()
    });

    fn low_s_bytes(length: usize) -> Vec<u8> {
        let mut bytes = vec![0u8; length];
        bytes[length / 2 - 1] = 1;
        bytes[length - 1] = 1;

        bytes
    }

    fn bad_point_sec1(coordinate_length: usize) -> Vec<u8> {
        let mut sec1 = vec![0xFFu8; 1 + coordinate_length * 2];
        sec1[0] = 0x04;

        sec1
    }

    #[test]
    fn p256_rejects_malformed_coordinates() {
        let bad = Base64UrlUnpadded::encode_string(&[0xFFu8; 32]);

        assert_eq!(
            verify_signature("header.payload", &[0u8; 64], &bad, &bad, Curve::P256)
                .err()
                .unwrap()
                .to_string(),
            format!(
                "the jwk public key coordinates could not be built into a verifying key: {}",
                p256::ecdsa::VerifyingKey::from_sec1_bytes(&bad_point_sec1(32))
                    .err()
                    .unwrap()
            )
        );
    }

    #[test]
    fn p256_rejects_malformed_signature() {
        let keypair = &*P256_KEYPAIR;

        assert_eq!(
            verify_signature(
                "header.payload",
                &[0u8; 10],
                &keypair.public.x,
                &keypair.public.y,
                Curve::P256,
            )
            .err()
            .unwrap()
            .to_string(),
            format!(
                "the token signature is not a valid ecdsa signature: {}",
                p256::ecdsa::Signature::from_slice(&[0u8; 10])
                    .err()
                    .unwrap()
            )
        );
    }

    #[test]
    fn p384_rejects_malformed_coordinates() {
        let bad = Base64UrlUnpadded::encode_string(&[0xFFu8; 48]);

        assert_eq!(
            verify_signature("header.payload", &[0u8; 96], &bad, &bad, Curve::P384)
                .err()
                .unwrap()
                .to_string(),
            format!(
                "the jwk public key coordinates could not be built into a verifying key: {}",
                p384::ecdsa::VerifyingKey::from_sec1_bytes(&bad_point_sec1(48))
                    .err()
                    .unwrap()
            )
        );
    }

    #[test]
    fn p384_rejects_malformed_signature() {
        let keypair = &*P384_KEYPAIR;

        assert_eq!(
            verify_signature(
                "header.payload",
                &[0u8; 10],
                &keypair.public.x,
                &keypair.public.y,
                Curve::P384,
            )
            .err()
            .unwrap()
            .to_string(),
            format!(
                "the token signature is not a valid ecdsa signature: {}",
                p384::ecdsa::Signature::from_slice(&[0u8; 10])
                    .err()
                    .unwrap()
            )
        );
    }

    #[test]
    fn p384_rejects_high_s_signature() {
        let keypair = &*P384_KEYPAIR;
        let signature = p384::ecdsa::Signature::from_slice(&low_s_bytes(96)).unwrap();
        let (r, s) = signature.split_scalars();
        let high_s = p384::ecdsa::Signature::from_scalars(r, -s)
            .unwrap()
            .to_bytes()
            .to_vec();

        assert_eq!(
            verify_signature(
                "header.payload",
                &high_s,
                &keypair.public.x,
                &keypair.public.y,
                Curve::P384,
            )
            .err()
            .unwrap()
            .to_string(),
            "the token signature is not in canonical low-s form"
        );
    }

    #[test]
    fn p384_reports_mismatch_for_non_matching_signature() {
        let keypair = &*P384_KEYPAIR;

        assert!(
            !verify_signature(
                "header.payload",
                &low_s_bytes(96),
                &keypair.public.x,
                &keypair.public.y,
                Curve::P384,
            )
            .unwrap()
        );
    }

    #[test]
    fn rejects_malformed_y_coordinate() {
        let keypair = &*P256_KEYPAIR;

        assert_eq!(
            verify_signature(
                "header.payload",
                &[0u8; 64],
                &keypair.public.x,
                "invalid @@@",
                Curve::P256,
            )
            .err()
            .unwrap()
            .to_string(),
            format!(
                "the jwk public key coordinate is not valid base64url: {}",
                Base64UrlUnpadded::decode_vec("invalid @@@").err().unwrap()
            )
        );
    }
}
