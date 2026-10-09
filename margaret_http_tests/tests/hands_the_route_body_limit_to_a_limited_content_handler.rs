use std::sync::Arc;

use margaret_http::body_limit::BodyLimit;
use margaret_http::limited_content_handler::limited_content_handler;
use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::limit_reporting_handler::LimitReportingHandler;
use margaret_http_tests::raw_exchange::raw_exchange;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_uploaded_file::upload_config::UploadConfig;
use margaret_route_method::content_method::ContentMethod;

#[tokio::test]
async fn hands_the_route_body_limit_to_a_limited_content_handler() {
    let server = RunningFixtureServer::start_plain(
        UploadConfig::Disabled,
        vec![RouteEntry::new(
            "/form",
            vec![MethodHandler::content(
                ContentMethod::Post,
                limited_content_handler(Arc::new(LimitReportingHandler), BodyLimit::new(4)),
            )],
        )],
    )
    .await;

    let response = raw_exchange(server.address(), b"POST /form HTTP/1.1\r\nHost: fixture.test\r\nContent-Length: 0\r\nConnection: close\r\n\r\n").await;

    assert!(response.starts_with("HTTP/1.1 200 "));
    assert!(response.ends_with("\r\n\r\n4"));

    server.stop().await;
}
