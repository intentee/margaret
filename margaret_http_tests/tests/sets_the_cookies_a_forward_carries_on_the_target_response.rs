use std::sync::Arc;

use cookie::Cookie;

use margaret_http::forwardable_route::ForwardableRoute;
use margaret_http::get_route_reference::GetRouteReference;
use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::cookie_changing_forward_handler::CookieChangingForwardHandler;
use margaret_http_tests::fixture_forward_target::FixtureForwardTarget;
use margaret_http_tests::fixture_forward_targets::FixtureForwardTargets;
use margaret_http_tests::path_param_echo_handler::PathParamEchoHandler;
use margaret_http_tests::raw_exchange::raw_exchange;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test]
async fn sets_the_cookies_a_forward_carries_on_the_target_response() {
    let target = Arc::new(StaticHandler {
        body: b"target".to_vec(),
        content_type: "text/plain",
        status: 222,
    });
    let server = RunningFixtureServer::start_plain_forwarding(
        ["/origin", "/target"],
        [
            RouteEntry::new(vec![MethodHandler::head(
                RouteMethod::Get,
                Arc::new(CookieChangingForwardHandler {
                    cookie: Cookie::new("visited", "origin"),
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

    assert!(response.starts_with("HTTP/1.1 222 "));
    assert!(response.contains("set-cookie: visited=origin\r\n"));
    assert!(response.ends_with("target"));

    server.stop().await;
}
