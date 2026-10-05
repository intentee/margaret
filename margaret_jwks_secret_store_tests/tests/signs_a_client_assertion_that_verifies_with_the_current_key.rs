use uuid::Uuid;

use margaret_identity_session::client_assertion_claims::ClientAssertionClaims;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_secret_store_tests::rolled_store::rolled_store;
use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_jws_verification::header_type::HeaderType;
use margaret_jws_verification::jws_verification::JwsVerification;
use margaret_jws_verification::key_set_assembly::KeySetAssembly;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;

#[test]
fn signs_a_client_assertion_that_verifies_with_the_current_key() {
    let secret = fresh_p256_secret();
    let KeySetAssembly::Assembled(key_set) =
        VerificationKeySet::assemble(vec![secret.current().verification_key().clone()])
    else {
        panic!("the current key forms a key set");
    };
    let assertion = rolled_store(secret).sign_client_assertion(&ClientAssertionClaims {
        audience: "https://issuer.example"
            .parse()
            .expect("the issuer is an identifier"),
        client_id: "client".parse().expect("the client identifier is visible"),
        expires_at: NumericDate::new(1_060),
        issued_at: NumericDate::new(1_000),
        jti: Uuid::from_u128(7),
    });
    let CompactJwsParsing::Parsed(jws) = CompactJws::parse(&assertion) else {
        panic!("the client assertion is a compact jws");
    };

    assert_eq!(
        jws.typ(),
        Some(&HeaderType::Supported(JwtType::ClientAuthentication))
    );
    assert!(matches!(key_set.verify(&jws), JwsVerification::Verified(_)));
}
