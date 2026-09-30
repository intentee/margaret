use std::sync::Arc;

use margaret_http::body_limit::BodyLimit;
use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::json_reading_handler::JsonReadingHandler;
use margaret_http_tests::raw_exchange::raw_exchange;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_uploaded_file::upload_config::UploadConfig;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test]
async fn refuses_a_body_declared_larger_than_the_route_limit_before_reading_it() {
    let server = RunningFixtureServer::start_plain(
        UploadConfig::Disabled,
        vec![RouteEntry::new(
            "/json",
            vec![MethodHandler::anonymous(
                RouteMethod::Post,
                Arc::new(JsonReadingHandler {
                    limit: BodyLimit::new(4),
                }),
            )],
        )],
    )
    .await;

    let response = raw_exchange(server.address(), b"POST /json HTTP/1.1\r\nHost: fixture.test\r\nContent-Type: application/json\r\nContent-Length: 64\r\nExpect: 100-continue\r\nConnection: close\r\n\r\n").await;

    assert!(response.starts_with("HTTP/1.1 413 "));
    assert!(!response.contains("100 Continue"));

    server.stop().await;
}
