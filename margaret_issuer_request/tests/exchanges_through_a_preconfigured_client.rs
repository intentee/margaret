use std::sync::Arc;

use reqwest::Method;
use reqwest::Request;

use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::fixture_client_builder::fixture_client_builder;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_http_tests::tls_fixture::TlsFixture;
use margaret_issuer_request::issuer_answer::IssuerAnswer;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test]
async fn exchanges_through_a_preconfigured_client() {
    let fixture = TlsFixture::generate();
    let server = RunningFixtureServer::start(
        fixture.server_config.clone(),
        vec![RouteEntry::new(
            "/refresh",
            vec![MethodHandler::head(
                RouteMethod::Get,
                Arc::new(StaticHandler {
                    body: b"refreshed".to_vec(),
                    content_type: "text/plain",
                    status: 200,
                }),
            )],
        )],
    )
    .await;
    let client = IssuerRequestClient::preconfigured(
        fixture_client_builder(&fixture.certificate_authority)
            .build()
            .expect("the preconfigured client builds"),
    );

    let Ok(IssuerAnswer::Received(answer)) = client
        .exchange(Request::new(
            Method::GET,
            fixture.url(server.port(), "/refresh"),
        ))
        .await
    else {
        panic!("the issuer answers");
    };

    server.stop().await;

    assert_eq!(answer.body(), b"refreshed");
}
