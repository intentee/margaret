use std::sync::Arc;

use oauth2::AuthorizationCode;
use oauth2::EmptyExtraTokenFields;
use oauth2::PkceCodeVerifier;
use oauth2::RedirectUrl;
use oauth2::TokenResponse;
use serde_json::Value;
use serde_json::json;
use url::Url;

use margaret_authorization_server_client::endpoint_outcome::EndpointOutcome;
use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_authorization_server_client_tests::secret_basic_client::secret_basic_client;
use margaret_http::body_limit::BodyLimit;
use margaret_http_tests::echo_wrapping::EchoWrapping;
use margaret_http_tests::form_echo_handler::FormEchoHandler;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test]
async fn exchanges_an_authorization_code() {
    let server = FixtureAuthorizationServer::start(
        RouteMethod::Post,
        "/token",
        Arc::new(FormEchoHandler {
            limit: BodyLimit::new(1024),
            wrapping: EchoWrapping::AccessToken,
        }),
    )
    .await;

    let outcome = server
        .client(Arc::new(secret_basic_client()))
        .exchange_authorization_code::<EmptyExtraTokenFields>(
            AuthorizationCode::new("SplxlOBeZQQYbYS6WxSbIA".to_string()),
            PkceCodeVerifier::new("dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk".to_string()),
            RedirectUrl::from_url(
                Url::parse("https://client.example/callback").expect("the callback is a url"),
            ),
        )
        .await
        .expect("a secret basic client needs no assertion");

    server.stop().await;

    let EndpointOutcome::Answered(response) = outcome else {
        panic!("the token endpoint answers");
    };
    let echoed: Value =
        serde_json::from_str(response.access_token().secret()).expect("the echo is json");

    assert_eq!(
        echoed,
        json!({
            "authorization": "Basic Y2xpZW50JTNBaWQ6czNjcmV0JTJGJTJCJTNE",
            "fields": {
                "code": "SplxlOBeZQQYbYS6WxSbIA",
                "code_verifier": "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk",
                "grant_type": "authorization_code",
                "redirect_uri": "https://client.example/callback",
            },
        })
    );
}
