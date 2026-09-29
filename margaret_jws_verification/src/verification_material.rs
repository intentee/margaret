use std::ops::ControlFlow;

use aws_lc_rs::encoding::AsDer;
use aws_lc_rs::signature::ParsedPublicKey;
use aws_lc_rs::signature::RSA_PKCS1_2048_8192_SHA256;
use aws_lc_rs::signature::RsaPublicKeyComponents;
use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;
use p256::ecdsa::signature::Verifier;
use p256::pkcs8::DecodePublicKey;

use margaret_jose_parameters::curve::Curve;
use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;

use crate::certificate_rejection::CertificateRejection;
use crate::key_material_rejection::KeyMaterialRejection;
use crate::rs256_public_key::Rs256PublicKey;
use crate::signature_check::SignatureCheck;

const SEC1_UNCOMPRESSED_POINT_TAG: u8 = 0x04;

fn coordinate_length(curve: Curve) -> usize {
    match curve {
        Curve::P256 => p256::FieldBytes::default().len(),
        Curve::P384 => p384::FieldBytes::default().len(),
    }
}

fn decoded(
    encoded: &str,
    rejection: impl FnOnce(base64ct::Error) -> KeyMaterialRejection,
) -> ControlFlow<KeyMaterialRejection, Vec<u8>> {
    match Base64UrlUnpadded::decode_vec(encoded) {
        Ok(bytes) => ControlFlow::Continue(bytes),
        Err(source) => ControlFlow::Break(rejection(source)),
    }
}

fn coordinate(curve: Curve, encoded: &str) -> ControlFlow<KeyMaterialRejection, Vec<u8>> {
    let bytes = decoded(encoded, |source| KeyMaterialRejection::CoordinateBase64 {
        source,
    })?;
    let expected = coordinate_length(curve);

    if bytes.len() != expected {
        return ControlFlow::Break(KeyMaterialRejection::CoordinateLength {
            expected,
            found: bytes.len(),
        });
    }

    ControlFlow::Continue(bytes)
}

fn significant_bits(big_endian: &[u8]) -> u64 {
    big_endian.iter().fold(0, |bits, octet| {
        if bits == 0 {
            u64::from(u8::BITS - octet.leading_zeros())
        } else {
            bits + u64::from(u8::BITS)
        }
    })
}

fn ecdsa_verification<Signature, Key: Verifier<Signature>>(
    key: &Key,
    message: &[u8],
    signature: Result<Signature, p256::ecdsa::Error>,
) -> SignatureCheck {
    match signature {
        Ok(signature) => match key.verify(message, &signature) {
            Ok(()) => SignatureCheck::Matches,
            Err(source) => SignatureCheck::EcdsaMismatch(source),
        },
        Err(source) => SignatureCheck::EcdsaMalformed(source),
    }
}

fn certified_ec_key<TVerifyingKey: DecodePublicKey + PartialEq>(
    key: &TVerifyingKey,
    subject_public_key_info: &[u8],
) -> ControlFlow<CertificateRejection> {
    match TVerifyingKey::from_public_key_der(subject_public_key_info) {
        Ok(certified) if certified == *key => ControlFlow::Continue(()),
        Ok(_) => ControlFlow::Break(CertificateRejection::KeyMismatch),
        Err(source) => {
            ControlFlow::Break(CertificateRejection::CertificateKeyUnreadable { source })
        }
    }
}

#[derive(Clone)]
pub enum VerificationMaterial {
    P256(p256::ecdsa::VerifyingKey),
    P384(p384::ecdsa::VerifyingKey),
    Rs256(Rs256PublicKey),
}

impl VerificationMaterial {
    pub(crate) fn from_coordinates(
        curve: Curve,
        x: &str,
        y: &str,
    ) -> ControlFlow<KeyMaterialRejection, Self> {
        let x = coordinate(curve, x)?;
        let y = coordinate(curve, y)?;
        let mut sec1 = vec![SEC1_UNCOMPRESSED_POINT_TAG];

        sec1.extend_from_slice(&x);
        sec1.extend_from_slice(&y);

        let material = match curve {
            Curve::P256 => p256::ecdsa::VerifyingKey::from_sec1_bytes(&sec1).map(Self::P256),
            Curve::P384 => p384::ecdsa::VerifyingKey::from_sec1_bytes(&sec1).map(Self::P384),
        };

        match material {
            Ok(material) => ControlFlow::Continue(material),
            Err(source) => ControlFlow::Break(KeyMaterialRejection::InvalidPoint { source }),
        }
    }

    pub(crate) fn from_rsa_components(n: &str, e: &str) -> ControlFlow<KeyMaterialRejection, Self> {
        let modulus = decoded(n, |source| KeyMaterialRejection::ModulusBase64 { source })?;
        let exponent = decoded(e, |source| KeyMaterialRejection::ExponentBase64 { source })?;
        let bits = significant_bits(&modulus);

        if bits < u64::from(RSA_PKCS1_2048_8192_SHA256.min_modulus_len())
            || bits > u64::from(RSA_PKCS1_2048_8192_SHA256.max_modulus_len())
        {
            return ControlFlow::Break(KeyMaterialRejection::ModulusSize { bits });
        }

        let components = RsaPublicKeyComponents {
            n: modulus.as_slice(),
            e: exponent.as_slice(),
        };
        let public_key = match components.as_der() {
            Ok(public_key) => public_key,
            Err(source) => {
                return ControlFlow::Break(KeyMaterialRejection::RsaComponentsNotMinimal {
                    source,
                });
            }
        };

        match ParsedPublicKey::new(&RSA_PKCS1_2048_8192_SHA256, public_key.as_ref()) {
            Ok(key) => ControlFlow::Continue(Self::Rs256(Rs256PublicKey { key })),
            Err(source) => ControlFlow::Break(KeyMaterialRejection::InvalidRsaKey { source }),
        }
    }

    pub(crate) fn algorithm(&self) -> JwsAlgorithm {
        match self {
            Self::P256(_) => Curve::P256.algorithm(),
            Self::P384(_) => Curve::P384.algorithm(),
            Self::Rs256(_) => JwsAlgorithm::Rs256,
        }
    }

    pub(crate) fn attested_by(
        &self,
        subject_public_key_info: &[u8],
    ) -> ControlFlow<CertificateRejection> {
        match self {
            Self::P256(key) => certified_ec_key(key, subject_public_key_info),
            Self::P384(key) => certified_ec_key(key, subject_public_key_info),
            Self::Rs256(Rs256PublicKey { key }) => {
                if key.as_ref() == subject_public_key_info {
                    ControlFlow::Continue(())
                } else {
                    ControlFlow::Break(CertificateRejection::KeyMismatch)
                }
            }
        }
    }

    pub(crate) fn check(&self, message: &[u8], signature: &[u8]) -> SignatureCheck {
        match self {
            Self::P256(key) => {
                ecdsa_verification(key, message, p256::ecdsa::Signature::from_slice(signature))
            }
            Self::P384(key) => {
                ecdsa_verification(key, message, p384::ecdsa::Signature::from_slice(signature))
            }
            Self::Rs256(Rs256PublicKey { key }) => match key.verify_sig(message, signature) {
                Ok(()) => SignatureCheck::Matches,
                Err(source) => SignatureCheck::RsaMismatch(source),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use base64ct::Base64UrlUnpadded;
    use base64ct::Encoding;

    use margaret_jose_parameters::curve::Curve;

    use super::VerificationMaterial;
    use crate::signature_check::SignatureCheck;

    const RFC_7515_A2_N: &str = "ofgWCuLjybRlzo0tZWJjNiuSfb4p4fAkd_wWJcyQoTbji9k0l8W26mPddxHmfHQp-Vaw-4qPCJrcS2mJPMEzP1Pt0Bm4d4QlL-yRT-SFd2lZS-pCgNMsD1W_YpRPEwOWvG6b32690r2jZ47soMZo9wGzjb_7OMg0LOL-bSf63kpaSHSXndS5z5rexMdbBYUsLA9e-KXBdQOS-UTo7WTBEMa2R2CapHg665xsmtdVMTBQY4uDZlxvb3qCo5ZwKh9kG4LT6_I5IhlJH7aGhyxXFvUK-DWNmoudF8NAco9_h9iaGNj8q2ethFkMLs91kzk2PAcDTW9gb54h4FRWyuXpoQ";
    const RFC_7515_A2_E: &str = "AQAB";
    const RFC_7515_A2_SIGNING_INPUT: &str = "eyJhbGciOiJSUzI1NiJ9.eyJpc3MiOiJqb2UiLA0KICJleHAiOjEzMDA4MTkzODAsDQogImh0dHA6Ly9leGFtcGxlLmNvbS9pc19yb290Ijp0cnVlfQ";
    const RFC_7515_A2_SIGNATURE: &str = "cC4hiUPoj9Eetdgtv3hF80EGrhuB__dzERat0XF9g2VtQgr9PJbu3XOiZj5RZmh7AAuHIm4Bh-0Qc_lF5YKt_O8W2Fp5jujGbds9uJdbF9CUAr7t1dnZcAcQjbKBYNX4BAynRFdiuB--f_nZLgrnbyTyWzO75vRK5h6xBArLIARNPvkSjtQBMHlb1L07Qe7K0GarZRmB_eSN9383LcOLn6_dO--xi12jzDwusC-eOkHWEsqtFZESc6BfI7noOPqvhJ1phCnvWh6IeYI2w9QOYEUipUTI8np6LbgGY9Fs98rqVt5AXLIhWkWywlVmtVrBp0igcN_IoypGlUPQGe77Rw";
    const RFC_7515_A3_X: &str = "f83OJ3D2xF1Bg8vub9tLe1gHMzV76e8Tus9uPHvRVEU";
    const RFC_7515_A3_Y: &str = "x_FEzRu9m36HLN_tue659LNpXW6pCyStikYjKIWI5a0";
    const RFC_7515_A3_SIGNING_INPUT: &str = "eyJhbGciOiJFUzI1NiJ9.eyJpc3MiOiJqb2UiLA0KICJleHAiOjEzMDA4MTkzODAsDQogImh0dHA6Ly9leGFtcGxlLmNvbS9pc19yb290Ijp0cnVlfQ";
    const RFC_7515_A3_SIGNATURE: &str =
        "DtEhU3ljbEg8L38VWAfUAqOyKAM6-Xx-F4GawxaepmXFCgfTjDxw5djxLa8ISlSApmWQxfKTUJqPP3-Kg6NU1Q";

    fn rfc_7515_a3_key() -> VerificationMaterial {
        VerificationMaterial::from_coordinates(Curve::P256, RFC_7515_A3_X, RFC_7515_A3_Y)
            .continue_value()
            .expect("the RFC 7515 A.3 key is accepted")
    }

    fn outcome(key: &VerificationMaterial, signing_input: &str, signature: &[u8]) -> &'static str {
        match key.check(signing_input.as_bytes(), signature) {
            SignatureCheck::EcdsaMalformed(_) => "malformed",
            SignatureCheck::EcdsaMismatch(_) | SignatureCheck::RsaMismatch(_) => "mismatch",
            SignatureCheck::Matches => "matches",
        }
    }

    fn rfc_7515_a3_signature() -> Vec<u8> {
        Base64UrlUnpadded::decode_vec(RFC_7515_A3_SIGNATURE).expect("the RFC signature decodes")
    }

    #[test]
    fn verifies_the_rfc_7515_a2_example() {
        let key = VerificationMaterial::from_rsa_components(RFC_7515_A2_N, RFC_7515_A2_E)
            .continue_value()
            .expect("the RFC 7515 A.2 key is accepted");
        let signature = Base64UrlUnpadded::decode_vec(RFC_7515_A2_SIGNATURE)
            .expect("the RFC signature decodes");

        assert_eq!(
            outcome(&key, RFC_7515_A2_SIGNING_INPUT, &signature),
            "matches"
        );
    }

    #[test]
    fn verifies_the_rfc_7515_a3_example() {
        assert_eq!(
            outcome(
                &rfc_7515_a3_key(),
                RFC_7515_A3_SIGNING_INPUT,
                &rfc_7515_a3_signature()
            ),
            "matches"
        );
    }

    #[test]
    fn reports_the_rfc_7515_a3_signature_over_another_input_as_a_mismatch() {
        assert_eq!(
            outcome(
                &rfc_7515_a3_key(),
                "eyJhbGciOiJFUzI1NiJ9.e30",
                &rfc_7515_a3_signature()
            ),
            "mismatch"
        );
    }

    #[test]
    fn reports_a_truncated_signature_as_malformed() {
        assert_eq!(
            outcome(
                &rfc_7515_a3_key(),
                RFC_7515_A3_SIGNING_INPUT,
                rfc_7515_a3_signature()
                    .split_last()
                    .expect("the signature has bytes")
                    .1
            ),
            "malformed"
        );
    }
}
