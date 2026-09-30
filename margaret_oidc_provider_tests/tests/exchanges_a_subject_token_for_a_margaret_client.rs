use std::collections::BTreeSet;

use zeroize::Zeroizing;

use margaret_authorization_server_client::target_audience::TargetAudience;
use margaret_authorization_server_client::token_target::TokenTarget;
use margaret_token_exchange_client::exchanged_token::ExchangedToken;
use margaret_token_exchange_client::subject_token::SubjectToken;
use margaret_token_exchange_client::subject_token_type::SubjectTokenType;
use margaret_token_exchange_client::token_exchange::TokenExchange;

use crate::ci_issuer::CiIssuer;
use crate::ci_subject::CI_SUBJECT;
use crate::jwt_parts::JwtParts;
use crate::margaret_client::MargaretClient;
use crate::provider_fixture::ProviderFixture;

#[tokio::test]
async fn exchanges_a_subject_token_for_a_margaret_client() {
    let ci = CiIssuer::publishing_keys();
    let fixture = ProviderFixture::start(vec![ci.exchanger.clone()]).await;
    let client = MargaretClient::of_portal(&fixture).await;
    let ExchangedToken::Exchanged(access_token) = TokenExchange::create(client.server.clone())
        .exchange(
            &SubjectToken {
                token: Zeroizing::new(ci.token("intentee/margaret")),
                token_type: SubjectTokenType::IdToken,
            },
            &TokenTarget {
                audience: TargetAudience::Unspecified,
                scopes: BTreeSet::new(),
            },
        )
        .await
        .expect("the client authenticates")
    else {
        panic!("the provider exchanges the ci token for the portal");
    };

    assert_eq!(
        JwtParts::of(&serde_json::Value::String(access_token.secret().clone())).payload["sub"],
        CI_SUBJECT.to_string()
    );

    client.stop().await;
    fixture.stop().await;
}
