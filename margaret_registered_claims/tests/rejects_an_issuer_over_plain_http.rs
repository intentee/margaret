use margaret_registered_claims::issuer_identifier::IssuerIdentifier;
use margaret_registered_claims::registered_claims_error::RegisteredClaimsError;

#[test]
fn rejects_an_issuer_over_plain_http() {
    assert!(matches!(
        "http://issuer.example".parse::<IssuerIdentifier>(),
        Err(RegisteredClaimsError::IssuerNotHttps { scheme, .. }) if scheme == "http"
    ));
}
