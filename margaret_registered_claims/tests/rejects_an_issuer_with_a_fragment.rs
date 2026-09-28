use margaret_registered_claims::issuer_identifier::IssuerIdentifier;
use margaret_registered_claims::registered_claims_error::RegisteredClaimsError;

#[test]
fn rejects_an_issuer_with_a_fragment() {
    assert!(matches!(
        "https://issuer.example#tenant".parse::<IssuerIdentifier>(),
        Err(RegisteredClaimsError::IssuerHasFragment { .. })
    ));
}
