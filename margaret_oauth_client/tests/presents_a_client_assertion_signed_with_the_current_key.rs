use std::sync::Arc;

use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_secret_store_tests::fixture_roller::fixture_roller;
use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_jws_verification::header_type::HeaderType;
use margaret_jws_verification::jws_verification::JwsVerification;
use margaret_oauth_client::client_authentication::ClientAuthentication;
use margaret_oauth_client::presented_client_authentication::PresentedClientAuthentication;
use margaret_registered_claims::numeric_date::NumericDate;

#[test]
fn presents_a_client_assertion_signed_with_the_current_key() {
    let roller = fixture_roller();
    let PresentedClientAuthentication::Assertion(assertion) =
        ClientAuthentication::PrivateKeyJwt(Arc::clone(&roller)).presented_to(
            "https://issuer.example",
            "client",
            NumericDate::new(1_000),
            NumericDate::new(1_060),
        )
    else {
        panic!("a private_key_jwt client presents an assertion");
    };
    let CompactJwsParsing::Parsed(jws) = CompactJws::parse(&assertion) else {
        panic!("the client assertion is a compact jws");
    };

    assert_eq!(
        jws.typ(),
        Some(&HeaderType::Supported(JwtType::ClientAuthentication))
    );
    assert!(matches!(
        roller.jwks_secret_holder().get().key_set().verify(&jws),
        JwsVerification::Verified(_)
    ));
}
