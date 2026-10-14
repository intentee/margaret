use std::sync::Arc;

use margaret_http::forwardable_route::ForwardableRoute;
use margaret_http::get_route_reference::GetRouteReference;
use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::fixture_forward_target::FixtureForwardTarget;
use margaret_http_tests::fixture_forward_targets::FixtureForwardTargets;
use margaret_http_tests::forwarding_handler::ForwardingHandler;
use margaret_http_tests::path_param_echo_handler::PathParamEchoHandler;
use margaret_http_tests::raw_exchange::raw_exchange;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::see_other_handler::SeeOtherHandler;
use margaret_http_tests::target_route_props::TargetRouteProps;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test]
async fn answers_a_forward_whose_target_redirects_with_the_redirect() {
    let target = Arc::new(SeeOtherHandler {
        route: GetRouteReference::new("http://fixture.test/greeting".to_string()),
    });
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
            RouteEntry::new(vec![MethodHandler::head(RouteMethod::Get, target.clone())]),
        ],
        FixtureForwardTargets {
            article: Arc::new(PathParamEchoHandler { name: "article" }),
            target,
        },
    )
    .await;

    let response = raw_exchange(
        server.address(),
        b"GET /origin HTTP/1.1\r\nHost: fixture.test\r\nConnection: close\r\n\r\n",
    )
    .await;

    assert!(response.starts_with("HTTP/1.1 303 "));
    assert!(response.contains("location: http://fixture.test/greeting\r\n"));

    server.stop().await;
}
