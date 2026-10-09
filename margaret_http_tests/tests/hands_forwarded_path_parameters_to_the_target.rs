use std::collections::HashMap;
use std::sync::Arc;

use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::forwarding_handler::ForwardingHandler;
use margaret_http_tests::path_param_echo_handler::PathParamEchoHandler;
use margaret_http_tests::raw_exchange::raw_exchange;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_uploaded_file::upload_config::UploadConfig;

#[tokio::test]
async fn hands_forwarded_path_parameters_to_the_target() {
    let server = RunningFixtureServer::start_plain(
        UploadConfig::Disabled,
        vec![
            RouteEntry::new(
                "/origin",
                vec![MethodHandler::forwardable(
                    "origin",
                    Arc::new(ForwardingHandler {
                        path_params: HashMap::from([("article".to_string(), "7".to_string())]),
                        target: "article",
                    }),
                )],
            ),
            RouteEntry::new(
                "/articles/{article}",
                vec![MethodHandler::forwardable(
                    "article",
                    Arc::new(PathParamEchoHandler { name: "article" }),
                )],
            ),
        ],
    )
    .await;

    let response = raw_exchange(
        server.address(),
        b"GET /origin HTTP/1.1\r\nHost: fixture.test\r\nConnection: close\r\n\r\n",
    )
    .await;

    assert!(response.starts_with("HTTP/1.1 200 "));
    assert!(response.ends_with("\r\n\r\n7"));

    server.stop().await;
}
