use std::sync::Arc;

use cookie::Cookie;
use http::StatusCode;
use oauth2::AccessToken;

use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_authorization_server_client_tests::secret_basic_authentication::secret_basic_authentication;
use margaret_http::method_handler::MethodHandler;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_jwks_secret_store_tests::fixture_roller::fixture_roller;
use margaret_oidc_sign_in::signed_in::SignedIn;
use margaret_oidc_sign_in::userinfo_fetch::UserinfoFetch;
use margaret_oidc_sign_in_tests::email_claims::EmailClaims;
use margaret_oidc_sign_in_tests::fixture_sign_in_flow::fixture_sign_in_flow;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test]
async fn relays_a_refused_userinfo_request() {
    let server = FixtureAuthorizationServer::start(
        "/userinfo",
        MethodHandler::head(
            RouteMethod::Get,
            Arc::new(StaticHandler {
                body: b"{}".to_vec(),
                content_type: "application/json",
                status: 401,
            }),
        ),
    )
    .await;
    let flow = fixture_sign_in_flow(
        Arc::new(server.client(secret_basic_authentication())),
        fixture_roller().await,
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
        UserinfoFetch::Refused { status } if status == StatusCode::UNAUTHORIZED
    ));
}
