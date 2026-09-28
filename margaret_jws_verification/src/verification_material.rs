use std::ops::ControlFlow;

use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;
use p256::ecdsa::signature::Verifier;

use margaret_jose_parameters::curve::Curve;

use crate::jwk_rejection::JwkRejection;
use crate::signature_check::SignatureCheck;

const SEC1_UNCOMPRESSED_POINT_TAG: u8 = 0x04;

fn coordinate_length(curve: Curve) -> usize {
    match curve {
        Curve::P256 => p256::FieldBytes::default().len(),
        Curve::P384 => p384::FieldBytes::default().len(),
    }
}

fn coordinate(curve: Curve, encoded: &str) -> ControlFlow<JwkRejection, Vec<u8>> {
    let bytes = match Base64UrlUnpadded::decode_vec(encoded) {
        Ok(bytes) => bytes,
        Err(source) => return ControlFlow::Break(JwkRejection::CoordinateBase64 { source }),
    };
    let expected = coordinate_length(curve);

    if bytes.len() != expected {
        return ControlFlow::Break(JwkRejection::CoordinateLength {
            expected,
            found: bytes.len(),
        });
    }

    ControlFlow::Continue(bytes)
}

fn verification<Signature, Key: Verifier<Signature>>(
    key: &Key,
    message: &[u8],
    signature: Result<Signature, p256::ecdsa::Error>,
) -> SignatureCheck {
    match signature {
        Ok(signature) => match key.verify(message, &signature) {
            Ok(()) => SignatureCheck::Matches,
            Err(source) => SignatureCheck::Mismatch(source),
        },
        Err(source) => SignatureCheck::Malformed(source),
    }
}

#[derive(Clone)]
pub(crate) enum VerificationMaterial {
    P256(p256::ecdsa::VerifyingKey),
    P384(p384::ecdsa::VerifyingKey),
}

impl VerificationMaterial {
    pub(crate) fn from_coordinates(
        curve: Curve,
        x: &str,
        y: &str,
    ) -> ControlFlow<JwkRejection, Self> {
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
            Err(source) => ControlFlow::Break(JwkRejection::InvalidPoint { source }),
        }
    }

    pub(crate) fn check(&self, message: &[u8], signature: &[u8]) -> SignatureCheck {
        match self {
            Self::P256(key) => {
                verification(key, message, p256::ecdsa::Signature::from_slice(signature))
            }
            Self::P384(key) => {
                verification(key, message, p384::ecdsa::Signature::from_slice(signature))
            }
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

    fn outcome(signing_input: &str, signature: &[u8]) -> &'static str {
        match rfc_7515_a3_key().check(signing_input.as_bytes(), signature) {
            SignatureCheck::Malformed(_) => "malformed",
            SignatureCheck::Matches => "matches",
            SignatureCheck::Mismatch(_) => "mismatch",
        }
    }

    fn rfc_7515_a3_signature() -> Vec<u8> {
        Base64UrlUnpadded::decode_vec(RFC_7515_A3_SIGNATURE).expect("the RFC signature decodes")
    }

    #[test]
    fn verifies_the_rfc_7515_a3_example() {
        assert_eq!(
            outcome(RFC_7515_A3_SIGNING_INPUT, &rfc_7515_a3_signature()),
            "matches"
        );
    }

    #[test]
    fn reports_the_rfc_7515_a3_signature_over_another_input_as_a_mismatch() {
        assert_eq!(
            outcome("eyJhbGciOiJFUzI1NiJ9.e30", &rfc_7515_a3_signature()),
            "mismatch"
        );
    }

    #[test]
    fn reports_a_truncated_signature_as_malformed() {
        assert_eq!(
            outcome(
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
