use serde_json::json;

use margaret_jwt_verification::jwt_presentation::JwtPresentation;
use margaret_jwt_verification::presented_jwt::PresentedJwt;
use margaret_jwt_verification_tests::signed_claims::SignedClaims;

#[test]
fn exposes_the_claimed_issuer_of_a_presented_jwt() {
    let SignedClaims { token, .. } =
        SignedClaims::new(&json!({ "aud": "api", "iss": "portal", "exp": 1_000 }));
    let JwtPresentation::Presented(presented) = PresentedJwt::present(&token) else {
        panic!("the token is a jwt");
    };

    assert_eq!(presented.claimed_issuer(), "portal");
}
