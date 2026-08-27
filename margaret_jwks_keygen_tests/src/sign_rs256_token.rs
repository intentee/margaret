use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;
use ring::rand::SystemRandom;
use ring::signature::RSA_PKCS1_SHA256;
use ring::signature::RsaKeyPair;
use serde::Serialize;

const RSA_TEST_KEY_PKCS8: &[u8] = include_bytes!("rsa_test_key.pkcs8.der");

#[derive(Serialize)]
struct Rs256Header<'header> {
    alg: &'header str,
    kid: &'header str,
}

/// # Panics
///
/// Panics when the fixture key cannot be loaded, or when the claims cannot be signed.
#[must_use]
pub fn sign_rs256_token<TClaims: Serialize>(kid: &str, claims: &TClaims) -> String {
    let key_pair =
        RsaKeyPair::from_pkcs8(RSA_TEST_KEY_PKCS8).expect("the fixture key is a pkcs#8 rsa key");
    let header_json = serde_json::to_vec(&Rs256Header { alg: "RS256", kid })
        .expect("the fixture header serializes");
    let claims_json = serde_json::to_vec(claims).expect("the fixture claims serialize");
    let signing_input = format!(
        "{}.{}",
        Base64UrlUnpadded::encode_string(&header_json),
        Base64UrlUnpadded::encode_string(&claims_json),
    );
    let mut signature = vec![0u8; key_pair.public().modulus_len()];

    key_pair
        .sign(
            &RSA_PKCS1_SHA256,
            &SystemRandom::new(),
            signing_input.as_bytes(),
            &mut signature,
        )
        .expect("the fixture key signs the input");

    format!(
        "{signing_input}.{}",
        Base64UrlUnpadded::encode_string(&signature)
    )
}
