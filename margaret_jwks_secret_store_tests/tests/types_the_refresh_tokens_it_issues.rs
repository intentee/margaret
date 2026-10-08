use uuid::Uuid;

use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_jwks_secret_store_tests::fixture_secrets::fixture_secrets;
use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_jws_verification::header_type::HeaderType;
use margaret_token_signer_tests::fixture_issuance::fixture_issuance;
use margaret_token_signer_tests::unix_time::unix_time;

#[test]
fn types_the_refresh_tokens_it_issues() {
    let refresh_token = JwksSecretStore::create(fixture_secrets(), fixture_issuance())
        .issue_refresh_token(Uuid::from_u128(1), unix_time(500));
    let CompactJwsParsing::Parsed(jws) = CompactJws::parse(&refresh_token.signed_claims) else {
        panic!("the refresh token is a compact jws");
    };

    assert_eq!(jws.typ(), Some(&HeaderType::Supported(JwtType::Refresh)));
}
