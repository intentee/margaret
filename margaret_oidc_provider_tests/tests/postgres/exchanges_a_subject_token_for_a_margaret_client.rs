use std::collections::BTreeSet;

use zeroize::Zeroizing;

use margaret_authorization_server_client::target_audience::TargetAudience;
use margaret_authorization_server_client::token_target::TokenTarget;
use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_oauth_vocabulary::subject_token_type::SubjectTokenType;
use margaret_oidc_provider_tests::ci_issuer::CiIssuer;
use margaret_oidc_provider_tests::ci_subject::CI_SUBJECT;
use margaret_oidc_provider_tests::jws_claims::jws_claims;
use margaret_oidc_provider_tests::margaret_client::MargaretClient;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_token_exchange_client::exchanged_token::ExchangedToken;
use margaret_token_exchange_client::subject_token::SubjectToken;
use margaret_token_exchange_client::token_exchange::TokenExchange;

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
    else {
        panic!("the provider exchanges the ci token for the portal");
    };

    let CompactJwsParsing::Parsed(jws) = CompactJws::parse(access_token.secret()) else {
        panic!("the access token is a compact jws");
    };

    assert_eq!(jws_claims(&jws)["sub"], CI_SUBJECT.to_string());

    client.stop().await;
    fixture.stop().await;
}
