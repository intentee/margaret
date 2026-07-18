use margaret_jwks_key_gen::jwks_key_error::JwksKeyError;
use margaret_jwks_roller::roller_error::RollerError;

#[test]
fn roller_error_key_generation_reports_its_source() {
    let error =
        RollerError::KeyGeneration(JwksKeyError::MissingPublicKeyCoordinate { coordinate: "x" });

    assert_eq!(
        error.to_string(),
        "failed to generate a jwks signing key: the generated public key is missing its x coordinate"
    );
}
