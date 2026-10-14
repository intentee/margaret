use std::sync::Arc;

use margaret_http::forwardable_route::ForwardableRoute;
use margaret_http::get_route_reference::GetRouteReference;
use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::failing_handler::FailingHandler;
use margaret_http_tests::fixture_forward_target::FixtureForwardTarget;
use margaret_http_tests::fixture_forward_targets::FixtureForwardTargets;
use margaret_http_tests::forwarding_handler::ForwardingHandler;
use margaret_http_tests::path_param_echo_handler::PathParamEchoHandler;
use margaret_http_tests::raw_exchange::raw_exchange;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::target_route_props::TargetRouteProps;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test]
async fn answers_a_failing_forward_target_with_a_server_error() {
    let failing = Arc::new(FailingHandler);
    let server = RunningFixtureServer::start_plain_forwarding(
        ["/origin", "/target"],
        [
            RouteEntry::new(vec![MethodHandler::head(
                RouteMethod::Get,
                Arc::new(ForwardingHandler {
                    props: || TargetRouteProps,
                    route: ForwardableRoute::new(
                        GetRouteReference::new("http://fixture.test/target".to_string()),
                        FixtureForwardTarget::Target,
                    ),
                }),
            )]),
            RouteEntry::new(vec![MethodHandler::head(RouteMethod::Get, failing.clone())]),
        ],
        FixtureForwardTargets {
            article: Arc::new(PathParamEchoHandler { name: "article" }),
            target: failing,
        },
    )
    .await;

    let response = raw_exchange(
        server.address(),
        b"GET /origin HTTP/1.1\r\nHost: fixture.test\r\nConnection: close\r\n\r\n",
    )
    .await;

    assert!(response.starts_with("HTTP/1.1 500 "));

    server.stop().await;
}
