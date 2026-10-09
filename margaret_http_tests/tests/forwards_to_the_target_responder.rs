use std::collections::HashMap;
use std::sync::Arc;

use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::forwarding_handler::ForwardingHandler;
use margaret_http_tests::raw_exchange::raw_exchange;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_http_uploaded_file::upload_config::UploadConfig;

#[tokio::test]
async fn forwards_to_the_target_responder() {
    let server = RunningFixtureServer::start_plain(
        UploadConfig::Disabled,
        vec![
            RouteEntry::new(
                "/origin",
                vec![MethodHandler::forwardable(
                    "origin",
                    Arc::new(ForwardingHandler {
                        path_params: HashMap::new(),
                        target: "target",
                    }),
                )],
            ),
            RouteEntry::new(
                "/target",
                vec![MethodHandler::forwardable(
                    "target",
                    Arc::new(StaticHandler {
                        body: b"target".to_vec(),
                        content_type: "text/plain",
                        status: 222,
                    }),
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

    assert!(response.starts_with("HTTP/1.1 222 "));
    assert!(response.ends_with("target"));

    server.stop().await;
}
