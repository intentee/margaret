use std::convert::identity;
use std::ops::ControlFlow;

use aws_lc_rs::encoding::AsDer;
use aws_lc_rs::encoding::PublicKeyX509Der;
use aws_lc_rs::error::KeyRejected;
use aws_lc_rs::signature::ED25519_PUBLIC_KEY_LEN;
use aws_lc_rs::signature::ParsedPublicKey;
use aws_lc_rs::signature::RsaPublicKeyComponents;
use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;

use margaret_jose_parameters::curve::Curve;
use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;
use margaret_jose_parameters::octet_key_pair_curve::OctetKeyPairCurve;

use crate::certificate_rejection::CertificateRejection;
use crate::key_material_rejection::KeyMaterialRejection;
use crate::rsa_signature_scheme::RsaSignatureScheme;
use crate::signature_check::SignatureCheck;
use crate::signature_scheme::SignatureScheme;

const SEC1_UNCOMPRESSED_POINT_TAG: u8 = 0x04;
const ED25519_SIGNATURE_OCTETS: usize = 64;

fn flow<TValue, TFailure, TRejection>(
    result: Result<TValue, TFailure>,
    rejection: fn(TFailure) -> TRejection,
) -> ControlFlow<TRejection, TValue> {
    match result {
        Ok(value) => ControlFlow::Continue(value),
        Err(failure) => ControlFlow::Break(rejection(failure)),
    }
}

fn decoded(
    encoded: &str,
    rejection: fn(base64ct::Error) -> KeyMaterialRejection,
) -> ControlFlow<KeyMaterialRejection, Vec<u8>> {
    flow(Base64UrlUnpadded::decode_vec(encoded), rejection)
}

fn coordinate(encoded: &str, expected: usize) -> ControlFlow<KeyMaterialRejection, Vec<u8>> {
    let bytes = decoded(encoded, KeyMaterialRejection::CoordinateBase64)?;

    if bytes.len() != expected {
        return ControlFlow::Break(KeyMaterialRejection::CoordinateLength {
            expected,
            found: bytes.len(),
        });
    }

    ControlFlow::Continue(bytes)
}

fn identical_keys(
    published: &PublicKeyX509Der<'static>,
    certified: &PublicKeyX509Der<'static>,
) -> Result<(), CertificateRejection> {
    if published.as_ref() == certified.as_ref() {
        Ok(())
    } else {
        Err(CertificateRejection::KeyMismatch)
    }
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

#[derive(Clone)]
pub struct VerificationMaterial {
    key: ParsedPublicKey,
    scheme: SignatureScheme,
    signature_octets: usize,
}

impl VerificationMaterial {
    pub(crate) fn from_coordinates(
        curve: Curve,
        x: &str,
        y: &str,
    ) -> ControlFlow<KeyMaterialRejection, Self> {
        let x = coordinate(x, curve.coordinate_octets())?;
        let y = coordinate(y, curve.coordinate_octets())?;
        let mut point = vec![SEC1_UNCOMPRESSED_POINT_TAG];

        point.extend_from_slice(&x);
        point.extend_from_slice(&y);

        Self::from_ec_point(curve, &point).map_break(KeyMaterialRejection::InvalidPoint)
    }

    pub fn from_ec_point(
        curve: Curve,
        uncompressed_point: &[u8],
    ) -> ControlFlow<KeyRejected, Self> {
        Self::parse(
            SignatureScheme::Ecdsa(curve),
            uncompressed_point,
            2 * curve.coordinate_octets(),
        )
    }

    pub(crate) fn from_octet_key_pair(
        curve: OctetKeyPairCurve,
        x: &str,
    ) -> ControlFlow<KeyMaterialRejection, Self> {
        match curve {
            OctetKeyPairCurve::Ed25519 => Self::parse(
                SignatureScheme::Ed25519,
                &coordinate(x, ED25519_PUBLIC_KEY_LEN)?,
                ED25519_SIGNATURE_OCTETS,
            )
            .map_break(KeyMaterialRejection::InvalidOctetKeyPair),
        }
    }

    pub fn from_rs256_components(
        modulus: &[u8],
        exponent: &[u8],
    ) -> ControlFlow<KeyMaterialRejection, Self> {
        Self::from_rsa_integers(modulus, exponent, RsaSignatureScheme::Pkcs1Sha256)
    }

    pub(crate) fn from_rsa_components(
        n: &str,
        e: &str,
        scheme: RsaSignatureScheme,
    ) -> ControlFlow<KeyMaterialRejection, Self> {
        let modulus = decoded(n, KeyMaterialRejection::ModulusBase64)?;
        let exponent = decoded(e, KeyMaterialRejection::ExponentBase64)?;

        Self::from_rsa_integers(&modulus, &exponent, scheme)
    }

    fn from_rsa_integers(
        modulus: &[u8],
        exponent: &[u8],
        scheme: RsaSignatureScheme,
    ) -> ControlFlow<KeyMaterialRejection, Self> {
        let bits = significant_bits(modulus);
        let parameters = scheme.parameters();

        if bits < u64::from(parameters.min_modulus_len())
            || bits > u64::from(parameters.max_modulus_len())
        {
            return ControlFlow::Break(KeyMaterialRejection::ModulusSize { bits });
        }

        let public_key = flow(
            RsaPublicKeyComponents {
                n: modulus,
                e: exponent,
            }
            .as_der(),
            KeyMaterialRejection::RsaComponentsNotMinimal,
        )?;

        Self::parse(
            SignatureScheme::Rsa(scheme),
            public_key.as_ref(),
            modulus.len(),
        )
        .map_break(KeyMaterialRejection::InvalidRsaKey)
    }

    fn parse(
        scheme: SignatureScheme,
        public_key: &[u8],
        signature_octets: usize,
    ) -> ControlFlow<KeyRejected, Self> {
        match ParsedPublicKey::new(scheme.verification_algorithm(), public_key) {
            Ok(key) => ControlFlow::Continue(Self {
                key,
                scheme,
                signature_octets,
            }),
            Err(rejection) => ControlFlow::Break(rejection),
        }
    }

    pub(crate) fn admits(&self, algorithm: JwsAlgorithm) -> bool {
        SignatureScheme::of(algorithm) == self.scheme
    }

    pub(crate) fn algorithm(&self) -> JwsAlgorithm {
        self.scheme.algorithm()
    }

    pub(crate) fn attested_by(
        &self,
        subject_public_key_info: &[u8],
    ) -> ControlFlow<CertificateRejection> {
        let certified = flow(
            ParsedPublicKey::new(self.key.algorithm(), subject_public_key_info),
            CertificateRejection::CertificateKeyUnreadable,
        )?;
        flow(
            self.key
                .as_der()
                .map_err(CertificateRejection::PublishedKeyUnencodable)
                .and_then(|published| {
                    certified
                        .as_der()
                        .map_err(CertificateRejection::CertificateKeyUnencodable)
                        .and_then(|certified| identical_keys(&published, &certified))
                }),
            identity,
        )
    }

    pub(crate) fn check(&self, message: &[u8], signature: &[u8]) -> SignatureCheck {
        if signature.len() != self.signature_octets {
            return SignatureCheck::LengthMismatch {
                expected: self.signature_octets,
                found: signature.len(),
            };
        }

        match self.key.verify_sig(message, signature) {
            Ok(()) => SignatureCheck::Matches,
            Err(source) => SignatureCheck::Mismatch(source),
        }
    }
}

#[cfg(test)]
mod tests {
    use aws_lc_rs::error::Unspecified;
    use base64ct::Base64UrlUnpadded;
    use base64ct::Encoding;

    use margaret_jose_parameters::curve::Curve;
    use margaret_jose_parameters::octet_key_pair_curve::OctetKeyPairCurve;

    use super::VerificationMaterial;
    use crate::rsa_signature_scheme::RsaSignatureScheme;
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
    const RFC_8037_A4_X: &str = "11qYAYKxCrfVS_7TyWQHOg7hcvPapiMlrwIaaPcHURo";
    const RFC_8037_A4_SIGNING_INPUT: &str =
        "eyJhbGciOiJFZERTQSJ9.RXhhbXBsZSBvZiBFZDI1NTE5IHNpZ25pbmc";
    const RFC_8037_A4_SIGNATURE: &str =
        "hgyY0il_MGCjP0JzlnLWG1PPOt7-09PGcvMg3AIbQR6dWbhijcNR4ki4iylGjg5BhVsPt9g7sVvpAr_MuM0KAg";

    fn rfc_7515_a3_key() -> VerificationMaterial {
        VerificationMaterial::from_coordinates(Curve::P256, RFC_7515_A3_X, RFC_7515_A3_Y)
            .continue_value()
            .expect("the RFC 7515 A.3 key is accepted")
    }

    fn decoded(encoded: &str) -> Vec<u8> {
        Base64UrlUnpadded::decode_vec(encoded).expect("the RFC signature decodes")
    }

    #[test]
    fn verifies_the_rfc_7515_a2_example() {
        let key = VerificationMaterial::from_rsa_components(
            RFC_7515_A2_N,
            RFC_7515_A2_E,
            RsaSignatureScheme::Pkcs1Sha256,
        )
        .continue_value()
        .expect("the RFC 7515 A.2 key is accepted");

        assert_eq!(
            key.check(
                RFC_7515_A2_SIGNING_INPUT.as_bytes(),
                &decoded(RFC_7515_A2_SIGNATURE)
            ),
            SignatureCheck::Matches
        );
    }

    #[test]
    fn verifies_the_rfc_7515_a3_example() {
        assert_eq!(
            rfc_7515_a3_key().check(
                RFC_7515_A3_SIGNING_INPUT.as_bytes(),
                &decoded(RFC_7515_A3_SIGNATURE)
            ),
            SignatureCheck::Matches
        );
    }

    #[test]
    fn verifies_the_rfc_8037_a4_example() {
        let key =
            VerificationMaterial::from_octet_key_pair(OctetKeyPairCurve::Ed25519, RFC_8037_A4_X)
                .continue_value()
                .expect("the RFC 8037 A.4 key is accepted");

        assert_eq!(
            key.check(
                RFC_8037_A4_SIGNING_INPUT.as_bytes(),
                &decoded(RFC_8037_A4_SIGNATURE)
            ),
            SignatureCheck::Matches
        );
    }

    #[test]
    fn reports_the_rfc_7515_a3_signature_over_another_input_as_a_mismatch() {
        assert_eq!(
            rfc_7515_a3_key().check(b"eyJhbGciOiJFUzI1NiJ9.e30", &decoded(RFC_7515_A3_SIGNATURE)),
            SignatureCheck::Mismatch(Unspecified)
        );
    }

    #[test]
    fn reports_a_truncated_signature_by_its_length() {
        let signature = decoded(RFC_7515_A3_SIGNATURE);

        assert_eq!(
            rfc_7515_a3_key().check(RFC_7515_A3_SIGNING_INPUT.as_bytes(), &signature[1..]),
            SignatureCheck::LengthMismatch {
                expected: 64,
                found: 63
            }
        );
    }
}
