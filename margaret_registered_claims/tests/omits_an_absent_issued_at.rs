use margaret_registered_claims::audience_claim::AudienceClaim;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_registered_claims::registered_claims::RegisteredClaims;

#[test]
fn omits_an_absent_issued_at() {
    let members = RegisteredClaims {
        aud: AudienceClaim::Single("api".to_string()),
        exp: NumericDate::new(200),
        iat: None,
        iss: "https://issuer.example".to_string(),
        jti: None,
        nbf: None,
    }
    .to_json();

    assert!(!members.contains_key("iat"));
    assert_eq!(members["exp"], 200);
}
