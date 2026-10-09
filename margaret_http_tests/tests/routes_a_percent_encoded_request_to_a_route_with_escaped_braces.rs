use std::sync::Arc;

use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::raw_exchange::raw_exchange;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_http_uploaded_file::upload_config::UploadConfig;

#[tokio::test]
async fn routes_a_percent_encoded_request_to_a_route_with_escaped_braces() {
    let server = RunningFixtureServer::start_plain(
        UploadConfig::Disabled,
        vec![RouteEntry::new(
            "/oauth/{{token}}",
            vec![MethodHandler::forwardable(
                "escaped",
                Arc::new(StaticHandler {
                    body: b"escaped".to_vec(),
                    content_type: "text/plain",
                    status: 222,
                }),
            )],
        )],
    )
    .await;

    let response = raw_exchange(
        server.address(),
        b"GET /oauth/%7Btoken%7D HTTP/1.1\r\nHost: fixture.test\r\nConnection: close\r\n\r\n",
    )
    .await;

    assert!(response.starts_with("HTTP/1.1 222 "));
    assert!(response.ends_with("escaped"));

    server.stop().await;
}
