use std::collections::HashMap;
use std::sync::Arc;

use oauth2::CsrfToken;
use oauth2::PkceCodeChallenge;
use oauth2::PkceCodeVerifier;
use oauth2::RedirectUrl;
use oauth2::Scope;
use url::Url;

use margaret_authorization_server_client::authorization_request::AuthorizationRequest;
use margaret_authorization_server_client::authorization_url::AuthorizationUrl;
use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_authorization_server_client_tests::oauth_client_declaration::OAuthClientDeclaration;
use margaret_authorization_server_client_tests::secret_basic_client::secret_basic_client;
use margaret_http::method_handler::MethodHandler;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test]
async fn builds_an_authorization_url() {
    let server = FixtureAuthorizationServer::start(
        "/authorize",
        MethodHandler::head(
            RouteMethod::Get,
            Arc::new(StaticHandler {
                body: Vec::new(),
                content_type: "text/plain",
                status: 200,
            }),
        ),
    )
    .await;
    let pkce_challenge = PkceCodeChallenge::from_code_verifier_sha256(&PkceCodeVerifier::new(
        "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk".to_string(),
    ));

    let built = server
        .client(Arc::new(OAuthClientDeclaration {
            client: secret_basic_client(),
        }))
        .authorization_url(AuthorizationRequest {
            nonce: "n-0S6_WzA2Mj".to_string(),
            pkce_challenge,
            redirect_uri: RedirectUrl::from_url(
                Url::parse("https://client.example/callback").expect("the callback is a url"),
            ),
            scopes: vec![
                Scope::new("openid".to_string()),
                Scope::new("profile".to_string()),
            ],
            state: CsrfToken::new("af0ifjsldkj".to_string()),
        })
        .await;

    server.stop().await;

    let AuthorizationUrl::Built(url) = built else {
        panic!("the authorization url is built");
    };

    assert_eq!(url.origin().ascii_serialization(), "https://localhost");
    assert_eq!(url.path(), "/authorize");
    assert_eq!(
        url.query_pairs()
            .into_owned()
            .collect::<HashMap<String, String>>(),
        HashMap::from([
            ("client_id".to_string(), "client:id".to_string()),
            (
                "code_challenge".to_string(),
                "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM".to_string()
            ),
            ("code_challenge_method".to_string(), "S256".to_string()),
            ("nonce".to_string(), "n-0S6_WzA2Mj".to_string()),
            (
                "redirect_uri".to_string(),
                "https://client.example/callback".to_string()
            ),
            ("response_type".to_string(), "code".to_string()),
            ("scope".to_string(), "openid profile".to_string()),
            ("state".to_string(), "af0ifjsldkj".to_string()),
        ])
    );
}
