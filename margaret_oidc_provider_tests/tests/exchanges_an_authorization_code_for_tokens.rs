use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_jws_verification::header_type::HeaderType;
use margaret_jws_verification::parameter_value::ParameterValue;
use margaret_oauth_vocabulary::scope_list::ScopeList;
use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider_tests::end_user_subject::END_USER_SUBJECT;
use margaret_oidc_provider_tests::fixture_scopes::fixture_scopes;
use margaret_oidc_provider_tests::issued_code::issued_code;
use margaret_oidc_provider_tests::jws_claims::jws_claims;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::portal_tokens::portal_tokens;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;

#[tokio::test]
async fn exchanges_an_authorization_code_for_tokens() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let AuthorizationOutcome::Redirected(redirect) = fixture.authorized(&portal_parameters()).await
    else {
        panic!("the portal is issued a code");
    };
    let answer = portal_tokens(&fixture, &issued_code(&redirect)).await;
    let CompactJwsParsing::Parsed(access_token) = CompactJws::parse(answer.member("access_token"))
    else {
        panic!("the access token is a compact jws");
    };
    let CompactJwsParsing::Parsed(id_token) = CompactJws::parse(answer.member("id_token")) else {
        panic!("the id token is a compact jws");
    };
    let access_claims = jws_claims(&access_token);
    let id_claims = jws_claims(&id_token);

    assert_eq!(answer.header("cache-control"), "no-store");
    assert_eq!(answer.header("pragma"), "no-cache");
    assert_eq!(answer.body["token_type"], "bearer");
    assert_eq!(answer.body["scope"], "openid profile");
    assert_eq!(
        answer.body["refresh_token"].as_str().map(str::len),
        Some(43)
    );
    assert_eq!(
        access_token.typ(),
        Some(&HeaderType::Supported(JwtType::AccessToken))
    );
    assert_eq!(
        access_claims["aud"],
        serde_json::json!(["artifacts", "https://localhost"])
    );
    assert_eq!(access_claims["client_id"], "portal");
    assert_eq!(access_claims["scope"], "openid profile");
    assert_eq!(access_claims["sub"], END_USER_SUBJECT.to_string());
    assert_eq!(
        id_token.alg(),
        &ParameterValue::Supported(JwsAlgorithm::Rs256)
    );
    assert_eq!(id_claims["aud"], "portal");
    assert_eq!(id_claims["azp"], "portal");
    assert_eq!(id_claims["nonce"], "n-0S6_WzA2Mj");
    assert_eq!(id_claims["sub"], END_USER_SUBJECT.to_string());
    assert!(id_claims["auth_time"].is_i64());
    assert_eq!(
        answer
            .member("scope")
            .parse::<ScopeList>()
            .expect("the scope is a scope list")
            .scopes,
        fixture_scopes(&["openid", "profile"])
    );

    fixture.stop().await;
}
