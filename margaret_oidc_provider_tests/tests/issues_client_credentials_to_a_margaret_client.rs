use std::collections::BTreeSet;

use margaret_authorization_server_client::target_audience::TargetAudience;
use margaret_authorization_server_client::token_target::TokenTarget;
use margaret_client_credentials::acquired_token::AcquiredToken;
use margaret_client_credentials::client_credentials::ClientCredentials;

use crate::jwt_parts::JwtParts;
use crate::margaret_client::MargaretClient;
use crate::provider_fixture::ProviderFixture;

#[tokio::test]
async fn issues_client_credentials_to_a_margaret_client() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let client = MargaretClient::of_portal(&fixture).await;
    let AcquiredToken::Acquired(access_token) = ClientCredentials::create(client.server.clone())
        .access_token(&TokenTarget {
            audience: TargetAudience::Unspecified,
            scopes: BTreeSet::from(["profile".parse().expect("the scope is a scope token")]),
        })
        .await
        .expect("the client authenticates")
    else {
        panic!("the provider issues client credentials to the portal");
    };
    let payload = JwtParts::of(&serde_json::Value::String(access_token.secret().clone())).payload;

    assert_eq!(payload["sub"], "portal");
    assert_eq!(payload["scope"], "profile");

    client.stop().await;
    fixture.stop().await;
}
