use margaret_registered_claims::audience_claim::AudienceClaim;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_registered_claims::registered_claims::RegisteredClaims;

#[test]
fn includes_a_present_not_before() {
    let members = RegisteredClaims {
        aud: AudienceClaim::Single("api".to_string()),
        exp: NumericDate::new(200),
        iat: None,
        iss: "https://issuer.example".to_string(),
        jti: None,
        nbf: Some(NumericDate::new(100)),
    }
    .to_json();

    assert_eq!(members["nbf"], 100);
}
