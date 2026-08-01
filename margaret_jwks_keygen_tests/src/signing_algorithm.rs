use anyhow::Result;
use anyhow::anyhow;
use jsonwebtoken::Algorithm;
use jsonwebtoken::jwk::KeyAlgorithm;

/// # Errors
///
/// Returns an error when the published algorithm is not one margaret signs with.
pub fn signing_algorithm(key_algorithm: KeyAlgorithm) -> Result<Algorithm> {
    match key_algorithm {
        KeyAlgorithm::ES256 => Ok(Algorithm::ES256),
        KeyAlgorithm::ES384 => Ok(Algorithm::ES384),
        unsupported => Err(anyhow!(
            "the published alg '{unsupported}' is not one margaret signs with"
        )),
    }
}

#[cfg(test)]
mod tests {
    use jsonwebtoken::Algorithm;
    use jsonwebtoken::jwk::KeyAlgorithm;

    use super::signing_algorithm;

    #[test]
    fn maps_the_published_elliptic_curve_algorithms() {
        assert_eq!(
            signing_algorithm(KeyAlgorithm::ES256).expect("es256 is published by margaret"),
            Algorithm::ES256
        );
        assert_eq!(
            signing_algorithm(KeyAlgorithm::ES384).expect("es384 is published by margaret"),
            Algorithm::ES384
        );
    }

    #[test]
    fn refuses_an_algorithm_margaret_never_publishes() {
        assert_eq!(
            signing_algorithm(KeyAlgorithm::RSA_OAEP)
                .expect_err("an encryption algorithm cannot validate a signature")
                .to_string(),
            "the published alg 'RSA_OAEP' is not one margaret signs with"
        );
    }
}
