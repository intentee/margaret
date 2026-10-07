use std::sync::Arc;

use cookie::Cookie;
use oauth2::AccessToken;

use margaret_authorization_server_client::server_unavailability::ServerUnavailability;
use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_authorization_server_client_tests::secret_basic_authentication::secret_basic_authentication;
use margaret_http::method_handler::MethodHandler;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_jwks_secret_store_tests::fixture_roller::fixture_roller;
use margaret_oidc_sign_in::sign_in_flow::SignInFlow;
use margaret_oidc_sign_in::signed_in::SignedIn;
use margaret_oidc_sign_in::userinfo_fetch::UserinfoFetch;
use margaret_oidc_sign_in_tests::email_claims::EmailClaims;
use margaret_route_method::route_method::RouteMethod;

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
        Arc::new(server.client(secret_basic_authentication())),
        fixture_roller(),
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
