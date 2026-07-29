use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;
use p256::elliptic_curve::rand_core::OsRng;
use p256::elliptic_curve::sec1::ToEncodedPoint;
use p256::pkcs8::EncodePrivateKey;
use p256::pkcs8::LineEnding;
use zeroize::Zeroizing;

use crate::curve::Curve;
use crate::generate_keypair_params::GenerateKeypairParams;
use crate::jwk_pair::JwkPair;
use crate::jwk_public::JwkPublic;
use crate::jwk_signing::JwkSigning;
use crate::jwks_key_error::JwksKeyError;
use crate::key_type::KeyType;
use crate::key_use::KeyUse;

fn build_jwk_pair(
    crv: Curve,
    kid: String,
    pem: Result<String, p256::pkcs8::Error>,
    x: Option<&[u8]>,
    y: Option<&[u8]>,
) -> Result<JwkPair, JwksKeyError> {
    let pem = pem?;

    Ok(JwkPair {
        public: JwkPublic {
            crv,
            kid: kid.clone(),
            kty: KeyType::Ec,
            use_: KeyUse::Signature,
            x: encode_coordinate(x.as_deref(), "x")?,
            y: encode_coordinate(y.as_deref(), "y")?,
        },
        signing: JwkSigning {
            crv,
            kid,
            pem: Zeroizing::new(pem),
        },
    })
}

fn encode_coordinate(
    coordinate: Option<&[u8]>,
    name: &'static str,
) -> Result<String, JwksKeyError> {
    match coordinate {
        Some(bytes) => Ok(Base64UrlUnpadded::encode_string(bytes)),
        None => Err(JwksKeyError::MissingPublicKeyCoordinate { coordinate: name }),
    }
}

/// # Errors
///
/// Returns an error propagated from the work it performs.
pub fn generate_keypair(
    GenerateKeypairParams { crv, kid }: GenerateKeypairParams,
) -> Result<JwkPair, JwksKeyError> {
    match crv {
        Curve::P256 => {
            let secret_key = p256::SecretKey::random(&mut OsRng);
            let point = secret_key.public_key().to_encoded_point(false);

            build_jwk_pair(
                crv,
                kid,
                secret_key
                    .to_pkcs8_pem(LineEnding::LF)
                    .map(|pem| pem.to_string()),
                point.x().map(|coordinate| coordinate.as_slice()),
                point.y().map(|coordinate| coordinate.as_slice()),
            )
        }
        Curve::P384 => {
            let secret_key = p384::SecretKey::random(&mut OsRng);
            let point = secret_key.public_key().to_encoded_point(false);

            build_jwk_pair(
                crv,
                kid,
                secret_key
                    .to_pkcs8_pem(LineEnding::LF)
                    .map(|pem| pem.to_string()),
                point.x().map(|coordinate| coordinate.as_slice()),
                point.y().map(|coordinate| coordinate.as_slice()),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use p256::pkcs8::DecodePrivateKey;

    use super::build_jwk_pair;
    use crate::curve::Curve;

    #[test]
    fn errors_when_private_key_pem_encoding_fails() {
        let error = build_jwk_pair(
            Curve::P256,
            "kid".to_string(),
            Err(p256::SecretKey::from_pkcs8_pem("not a valid pem")
                .err()
                .unwrap()),
            Some(&[1]),
            Some(&[2]),
        )
        .err()
        .unwrap();
        let expected = format!(
            "the generated private key could not be encoded to pkcs#8 pem: {}",
            p256::SecretKey::from_pkcs8_pem("not a valid pem")
                .err()
                .unwrap()
        );

        assert_eq!(error.to_string(), expected);
    }

    #[test]
    fn errors_when_x_coordinate_missing() {
        let error = build_jwk_pair(
            Curve::P256,
            "kid".to_string(),
            Ok("pem".to_string()),
            None,
            Some(&[2]),
        )
        .err()
        .unwrap();

        assert_eq!(
            error.to_string(),
            "the generated public key is missing its x coordinate"
        );
    }

    #[test]
    fn errors_when_y_coordinate_missing() {
        let error = build_jwk_pair(
            Curve::P256,
            "kid".to_string(),
            Ok("pem".to_string()),
            Some(&[1]),
            None,
        )
        .err()
        .unwrap();

        assert_eq!(
            error.to_string(),
            "the generated public key is missing its y coordinate"
        );
    }
}
