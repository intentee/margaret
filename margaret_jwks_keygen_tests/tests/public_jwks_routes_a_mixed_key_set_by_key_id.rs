use anyhow::Result;

use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::generate_keypair::generate_keypair;
use margaret_jwks_keygen::generate_keypair_params::GenerateKeypairParams;
use margaret_jwks_keygen::jwk_public::JwkPublic;
use margaret_jwks_keygen::key_use::KeyUse;
use margaret_jwks_keygen::public_jwks::PublicJwks;
use margaret_jwks_keygen::rsa_algorithm::RsaAlgorithm;
use margaret_jwks_keygen::rsa_jwk_public::RsaJwkPublic;
use margaret_jwks_keygen::signs_claims::SignsClaims;
use margaret_jwks_keygen::verifies_token::VerifiesToken;
use margaret_jwks_keygen_tests::far_future_expiry::FAR_FUTURE_EXPIRY;
use margaret_jwks_keygen_tests::rsa_test_key_exponent::RSA_TEST_KEY_EXPONENT;
use margaret_jwks_keygen_tests::rsa_test_key_modulus::RSA_TEST_KEY_MODULUS;
use margaret_jwks_keygen_tests::sign_rs256_token::sign_rs256_token;
use margaret_jwks_keygen_tests::test_claims::TestClaims;

#[tokio::test]
async fn public_jwks_routes_a_mixed_key_set_by_key_id() -> Result<()> {
    let ec_keypair = generate_keypair(GenerateKeypairParams {
        crv: Curve::P256,
        kid: "ec-kid".to_string(),
    })?;
    let ec_claims = TestClaims {
        exp: FAR_FUTURE_EXPIRY,
        sub: "ec-subject".to_string(),
    };
    let rsa_claims = TestClaims {
        exp: FAR_FUTURE_EXPIRY,
        sub: "rsa-subject".to_string(),
    };
    let ec_token = ec_keypair.signing.sign(&ec_claims).await?;
    let rsa_token = sign_rs256_token("rsa-kid", &rsa_claims);
    let public_jwks = PublicJwks::new(vec![
        JwkPublic::Ec(ec_keypair.public),
        JwkPublic::Rsa(RsaJwkPublic {
            alg: RsaAlgorithm::Rs256,
            e: RSA_TEST_KEY_EXPONENT.to_string(),
            kid: "rsa-kid".to_string(),
            n: RSA_TEST_KEY_MODULUS.to_string(),
            use_: KeyUse::Signature,
        }),
    ])?;

    assert_eq!(
        public_jwks.verify::<TestClaims>(&ec_token)?.verified(),
        Some(ec_claims)
    );
    assert_eq!(
        public_jwks.verify::<TestClaims>(&rsa_token)?.verified(),
        Some(rsa_claims)
    );

    Ok(())
}
