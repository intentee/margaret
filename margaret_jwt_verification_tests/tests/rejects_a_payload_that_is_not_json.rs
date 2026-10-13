use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;
use serde_json::json;

use margaret_jws_verification_tests::signed_token::signed_token;
use margaret_jwt_verification::claims_rejection::ClaimsRejection;
use margaret_jwt_verification::jwt_presentation::JwtPresentation;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_jwt_verification::presented_jwt::PresentedJwt;

#[test]
fn rejects_a_payload_that_is_not_json() {
    let token = signed_token(
        &format!(
            "{}.{}",
            Base64UrlUnpadded::encode_string(
                json!({ "alg": "ES256", "kid": "kid" })
                    .to_string()
                    .as_bytes()
            ),
            Base64UrlUnpadded::encode_string(b"not json"),
        ),
        &[0; 64],
    );

    assert!(matches!(
        PresentedJwt::present(&token),
        JwtPresentation::Rejected(JwtRejection::Claims(ClaimsRejection::Malformed { .. }))
    ));
}
