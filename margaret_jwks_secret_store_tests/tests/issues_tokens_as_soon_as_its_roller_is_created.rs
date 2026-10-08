use uuid::Uuid;

use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_jwks_secret_store_tests::fixture_roller::fixture_roller;
use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_jws_verification::header_type::HeaderType;
use margaret_token_signer_tests::fixture_issuance::fixture_issuance;
use margaret_token_signer_tests::unix_time::unix_time;

#[tokio::test]
async fn issues_tokens_as_soon_as_its_roller_is_created() {
    let refresh_token = JwksSecretStore::create(fixture_roller().await, fixture_issuance())
        .issue_refresh_token(Uuid::from_u128(1), unix_time(500));
    let CompactJwsParsing::Parsed(jws) = CompactJws::parse(&refresh_token.signed_claims) else {
        panic!("the refresh token is a compact jws");
    };

    assert_eq!(jws.typ(), Some(&HeaderType::Supported(JwtType::Refresh)));
}
