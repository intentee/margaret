use std::sync::Arc;

use cookie::Cookie;
use oauth2::AccessToken;

use margaret_authorization_server_client::server_unavailability::ServerUnavailability;
use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_authorization_server_client_tests::oauth_client_declaration::OAuthClientDeclaration;
use margaret_authorization_server_client_tests::secret_basic_client::secret_basic_client;
use margaret_http::method_handler::MethodHandler;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_jwks_secret_store_tests::rolled_store::rolled_store;
use margaret_oidc_sign_in::sign_in_flow::SignInFlow;
use margaret_oidc_sign_in::signed_in::SignedIn;
use margaret_oidc_sign_in::userinfo_fetch::UserinfoFetch;
use margaret_oidc_sign_in_tests::email_claims::EmailClaims;
use margaret_route_method::route_method::RouteMethod;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;

#[tokio::test]
async fn reports_unavailable_userinfo() {
    let server = FixtureAuthorizationServer::start(
        "/userinfo",
        MethodHandler::head(
            RouteMethod::Get,
            Arc::new(StaticHandler {
                body: b"not json".to_vec(),
                content_type: "application/json",
                status: 200,
            }),
        ),
    )
    .await;
    let flow = SignInFlow::create(
        Arc::new(server.client(Arc::new(OAuthClientDeclaration {
            client: secret_basic_client(),
        }))),
        Arc::new(rolled_store(fresh_p256_secret())),
    );

    let fetched = flow
        .userinfo::<EmailClaims, ()>(&SignedIn {
            access_token: AccessToken::new("2YotnFZFEjr1zCsicMWpAA".to_string()),
            claims: (),
            subject: "subject".to_string(),
            transaction_removal: Cookie::new("transaction", ""),
        })
        .await;

    server.stop().await;

    assert!(matches!(
        fetched,
        UserinfoFetch::Unavailable(ServerUnavailability::MalformedAnswer { .. })
    ));
}
