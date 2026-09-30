use std::sync::Arc;

use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::failing_handler::FailingHandler;
use margaret_http_tests::raw_exchange::raw_exchange;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_uploaded_file::upload_config::UploadConfig;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test]
async fn hides_a_responder_failure_behind_a_generic_server_error() {
    let server = RunningFixtureServer::start_plain(
        UploadConfig::Disabled,
        vec![RouteEntry::new(
            "/failing",
            vec![MethodHandler::anonymous(
                RouteMethod::Get,
                Arc::new(FailingHandler),
            )],
        )],
    )
    .await;

    let response = raw_exchange(
        server.address(),
        b"GET /failing HTTP/1.1\r\nHost: fixture.test\r\nConnection: close\r\n\r\n",
    )
    .await;

    assert!(response.starts_with("HTTP/1.1 500 "));
    assert!(response.ends_with("Internal Server Error"));
    assert!(!response.contains("secret"));

    server.stop().await;
}
