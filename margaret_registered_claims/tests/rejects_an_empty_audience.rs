use margaret_registered_claims::audience::Audience;
use margaret_registered_claims::registered_claims_error::RegisteredClaimsError;

#[test]
fn rejects_an_empty_audience() {
    assert!(matches!(
        "".parse::<Audience>(),
        Err(RegisteredClaimsError::AudienceEmpty)
    ));
}
