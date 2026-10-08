use margaret_authorization_server_client::target_audience::TargetAudience;
use margaret_authorization_server_client::token_target::TokenTarget;
use margaret_client_credentials::acquired_token::AcquiredToken;
use margaret_client_credentials::client_credentials::ClientCredentials;
use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_oidc_provider_tests::fixture_scopes::fixture_scopes;
use margaret_oidc_provider_tests::jws_claims::jws_claims;
use margaret_oidc_provider_tests::margaret_client::MargaretClient;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;

#[tokio::test]
async fn issues_client_credentials_to_a_margaret_client() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let client = MargaretClient::of_portal(&fixture).await;
    let AcquiredToken::Acquired(access_token) = ClientCredentials::create(client.server.clone())
        .access_token(&TokenTarget {
            audience: TargetAudience::Unspecified,
            scopes: fixture_scopes(&["profile"]),
        })
        .await
    else {
        panic!("the provider issues client credentials to the portal");
    };
    let CompactJwsParsing::Parsed(jws) = CompactJws::parse(access_token.secret()) else {
        panic!("the access token is a compact jws");
    };
    let payload = jws_claims(&jws);

    assert_eq!(payload["sub"], "portal");
    assert_eq!(payload["scope"], "profile");

    client.stop().await;
    fixture.stop().await;
}
