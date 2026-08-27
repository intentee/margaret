use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;
use p256::ecdsa::signature::Verifier;

use crate::curve::Curve;
use crate::jwks_key_error::JwksKeyError;
use crate::signature_check::SignatureCheck;
use crate::token_malformation::TokenMalformation;

fn coordinate_bytes(coordinate: &str, crv: Curve) -> Result<Vec<u8>, JwksKeyError> {
    let bytes = Base64UrlUnpadded::decode_vec(coordinate)
        .map_err(|source| JwksKeyError::CoordinateBase64 { source })?;
    let expected = crv.coordinate_bytes();

    if bytes.len() != expected {
        return Err(JwksKeyError::CoordinateLength {
            expected,
            found: bytes.len(),
        });
    }

    Ok(bytes)
}

fn sec1_point(x: &str, y: &str, crv: Curve) -> Result<Vec<u8>, JwksKeyError> {
    let mut sec1 = vec![0x04u8];

    sec1.extend_from_slice(&coordinate_bytes(x, crv)?);
    sec1.extend_from_slice(&coordinate_bytes(y, crv)?);

    Ok(sec1)
}

pub(crate) fn verify_ec_signature(
    signing_input: &str,
    signature_bytes: &[u8],
    x: &str,
    y: &str,
    crv: Curve,
) -> Result<SignatureCheck, JwksKeyError> {
    let sec1 = sec1_point(x, y, crv)?;

    match crv {
        Curve::P256 => {
            let verifying_key = p256::ecdsa::VerifyingKey::from_sec1_bytes(&sec1)
                .map_err(|source| JwksKeyError::MalformedVerifyingKey { source })?;
            let signature = match p256::ecdsa::Signature::from_slice(signature_bytes) {
                Ok(signature) => signature,
                Err(source) => {
                    return Ok(SignatureCheck::Malformed(
                        TokenMalformation::SignatureMalformed(source),
                    ));
                }
            };

            if signature.normalize_s().is_some() {
                return Ok(SignatureCheck::Malformed(
                    TokenMalformation::NonCanonicalSignature,
                ));
            }

            Ok(SignatureCheck::from_verification(
                verifying_key
                    .verify(signing_input.as_bytes(), &signature)
                    .is_ok(),
            ))
        }
        Curve::P384 => {
            let verifying_key = p384::ecdsa::VerifyingKey::from_sec1_bytes(&sec1)
                .map_err(|source| JwksKeyError::MalformedVerifyingKey { source })?;
            let signature = match p384::ecdsa::Signature::from_slice(signature_bytes) {
                Ok(signature) => signature,
                Err(source) => {
                    return Ok(SignatureCheck::Malformed(
                        TokenMalformation::SignatureMalformed(source),
                    ));
                }
            };

            if signature.normalize_s().is_some() {
                return Ok(SignatureCheck::Malformed(
                    TokenMalformation::NonCanonicalSignature,
                ));
            }

            Ok(SignatureCheck::from_verification(
                verifying_key
                    .verify(signing_input.as_bytes(), &signature)
                    .is_ok(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use base64ct::Base64UrlUnpadded;
    use base64ct::Encoding;

    use super::verify_ec_signature;
    use crate::curve::Curve;
    use crate::generate_keypair::generate_keypair;
    use crate::generate_keypair_params::GenerateKeypairParams;
    use crate::jwk_pair::JwkPair;
    use crate::jwks_key_error::JwksKeyError;
    use crate::sign_signing_input::sign_signing_input;
    use crate::signature_check::SignatureCheck;
    use crate::token_malformation::TokenMalformation;

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

    fn describe(
        signature_bytes: &[u8],
        keypair: &JwkPair,
        crv: Curve,
    ) -> Result<String, JwksKeyError> {
        describe_input("header.payload", signature_bytes, keypair, crv)
    }

    fn describe_input(
        signing_input: &str,
        signature_bytes: &[u8],
        keypair: &JwkPair,
        crv: Curve,
    ) -> Result<String, JwksKeyError> {
        verify_ec_signature(
            signing_input,
            signature_bytes,
            &keypair.public.x,
            &keypair.public.y,
            crv,
        )
        .map(|check| match check {
            SignatureCheck::Malformed(malformation) => malformation.to_string(),
            SignatureCheck::Matches => "matches".to_string(),
            SignatureCheck::Mismatch => "mismatch".to_string(),
        })
    }

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
            verify_ec_signature("header.payload", &[0u8; 64], &bad, &bad, Curve::P256)
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
    fn p256_reports_a_malformed_signature() {
        assert_eq!(
            describe(&[0u8; 10], &P256_KEYPAIR, Curve::P256)
                .ok()
                .as_deref(),
            Some(
                TokenMalformation::SignatureMalformed(
                    p256::ecdsa::Signature::from_slice(&[0u8; 10])
                        .expect_err("the fixture is not a valid signature")
                )
                .to_string()
                .as_str()
            )
        );
    }

    #[test]
    fn p384_rejects_malformed_coordinates() {
        let bad = Base64UrlUnpadded::encode_string(&[0xFFu8; 48]);

        assert_eq!(
            verify_ec_signature("header.payload", &[0u8; 96], &bad, &bad, Curve::P384)
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
    fn p384_reports_a_malformed_signature() {
        assert_eq!(
            describe(&[0u8; 10], &P384_KEYPAIR, Curve::P384)
                .ok()
                .as_deref(),
            Some(
                TokenMalformation::SignatureMalformed(
                    p384::ecdsa::Signature::from_slice(&[0u8; 10])
                        .expect_err("the fixture is not a valid signature")
                )
                .to_string()
                .as_str()
            )
        );
    }

    #[test]
    fn p384_reports_a_high_s_signature_as_non_canonical() {
        let signature = p384::ecdsa::Signature::from_slice(&low_s_bytes(96))
            .expect("the low-s fixture is a valid signature");
        let (r, s) = signature.split_scalars();
        let high_s = p384::ecdsa::Signature::from_scalars(r, -s)
            .expect("negating s yields a valid signature")
            .to_bytes()
            .to_vec();

        assert_eq!(
            describe(&high_s, &P384_KEYPAIR, Curve::P384)
                .ok()
                .as_deref(),
            Some("the token signature is not in canonical low-s form")
        );
    }

    #[test]
    fn p384_reports_mismatch_for_non_matching_signature() {
        assert_eq!(
            describe(&low_s_bytes(96), &P384_KEYPAIR, Curve::P384)
                .ok()
                .as_deref(),
            Some("mismatch")
        );
    }

    #[test]
    fn rejects_malformed_y_coordinate() {
        let keypair = &*P256_KEYPAIR;

        assert_eq!(
            verify_ec_signature(
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

    #[test]
    fn p256_reports_a_matching_signature() {
        let keypair = &*P256_KEYPAIR;
        let signing_input = "header.payload";
        let signature_segment =
            sign_signing_input(signing_input, &keypair.signing.pem, Curve::P256)
                .expect("the signing input is signed");
        let signature_bytes = Base64UrlUnpadded::decode_vec(&signature_segment)
            .expect("the signature segment decodes");

        assert_eq!(
            describe_input(signing_input, &signature_bytes, keypair, Curve::P256)
                .ok()
                .as_deref(),
            Some("matches")
        );
    }
}
