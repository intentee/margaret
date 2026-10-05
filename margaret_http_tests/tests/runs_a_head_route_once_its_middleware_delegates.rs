use std::sync::Arc;

use margaret_http::head_handler::HeadHandler;
use margaret_http::layer::layer;
use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::delegating_middleware::DelegatingMiddleware;
use margaret_http_tests::raw_exchange::raw_exchange;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_http_uploaded_file::upload_config::UploadConfig;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test]
async fn runs_a_head_route_once_its_middleware_delegates() {
    let page: Arc<dyn HeadHandler> = Arc::new(StaticHandler {
        body: b"page".to_vec(),
        content_type: "text/plain",
        status: 200,
    });
    let server = RunningFixtureServer::start_plain(
        UploadConfig::Disabled,
        vec![RouteEntry::new(
            "/page",
            vec![MethodHandler::head(
                RouteMethod::Get,
                layer(Arc::new(DelegatingMiddleware), page),
            )],
        )],
    )
    .await;

    let response = raw_exchange(
        server.address(),
        b"GET /page HTTP/1.1\r\nHost: fixture.test\r\nConnection: close\r\n\r\n",
    )
    .await;

    assert!(response.starts_with("HTTP/1.1 200 "));
    assert!(response.ends_with("page"));

    server.stop().await;
}
