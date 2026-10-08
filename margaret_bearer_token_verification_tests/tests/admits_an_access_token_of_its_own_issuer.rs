use std::sync::Arc;

use margaret_bearer_token_verification_tests::admit_access_token::admit_access_token;
use margaret_http::token_admission::TokenAdmission;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_jwks_secret_store_tests::fixture_secrets::fixture_secrets;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_token_signer_tests::fixture_issuance::fixture_issuance;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

#[tokio::test]
async fn admits_an_access_token_of_its_own_issuer() {
    let secrets = fixture_secrets();
    let token = TestClaims {
        sub: "subject".to_string(),
    }
    .signed_by(secrets.get().current());
    let trusted_issuer = TrustedIssuer::own(
        Arc::new(JwksSecretStore::create(secrets, fixture_issuance())),
        fixture_trust(),
    );

    let TokenAdmission::Admitted(verified) =
        admit_access_token(&trusted_issuer, &format!("Bearer {token}")).await
    else {
        panic!("the token signed with the own keys is admitted");
    };

    assert_eq!(verified.claims.sub, "subject");
}
