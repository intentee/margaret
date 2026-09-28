use margaret_registered_claims::issuer_identifier::IssuerIdentifier;
use margaret_registered_claims::registered_claims_error::RegisteredClaimsError;

#[test]
fn rejects_an_issuer_that_is_not_a_url() {
    assert!(matches!(
        "issuer.example".parse::<IssuerIdentifier>(),
        Err(RegisteredClaimsError::IssuerMalformed { .. })
    ));
}
