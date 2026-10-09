use std::sync::Arc;

use http::header::COOKIE;
use http::header::SET_COOKIE;

use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_uploaded_file::upload_config::UploadConfig;
use margaret_oidc_provider_tests::fixture_authorization_handler::fixture_authorization_handler;
use margaret_oidc_provider_tests::fixture_consent_view::FixtureConsentView;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::signed_in_session_cookies::signed_in_session_cookies;
use margaret_oidc_provider_tests::spa_parameters::spa_parameters;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test]
async fn renders_the_consent_page_of_a_signed_in_end_user() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let secret = signed_in_session_cookies(&fixture.sessions)
        .await
        .into_iter()
        .find(|cookie| cookie.name() == "__Host-margaret-session")
        .expect("the session secret is set");
    let server = RunningFixtureServer::start_plain(
        UploadConfig::Disabled,
        vec![RouteEntry::new(
            "/authorize",
            vec![MethodHandler::head(
                RouteMethod::Get,
                Arc::new(fixture_authorization_handler(&fixture, FixtureConsentView)),
            )],
        )],
    )
    .await;
    let response = reqwest::Client::new()
        .get(format!("http://{}/authorize", server.address()))
        .query(&spa_parameters())
        .header(COOKIE, format!("{}={}", secret.name(), secret.value()))
        .send()
        .await
        .expect("the authorization page answers");
    let status = response.status().as_u16();
    let refreshed = response.headers().contains_key(SET_COOKIE);
    let page = response.text().await.expect("the page is read");

    server.stop().await;
    fixture.stop().await;

    assert_eq!(status, 200);
    assert!(refreshed);
    assert!(page.contains("action=\"https://issuer.fixture/authorize/consent\""));
    assert!(page.contains("href=\"https://issuer.fixture/\""));
}
