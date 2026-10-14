use std::sync::Arc;

use margaret_http::forwardable_route::ForwardableRoute;
use margaret_http::get_route_reference::GetRouteReference;
use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::article_route_props::ArticleRouteProps;
use margaret_http_tests::fixture_forward_target::FixtureForwardTarget;
use margaret_http_tests::fixture_forward_targets::FixtureForwardTargets;
use margaret_http_tests::forwarding_handler::ForwardingHandler;
use margaret_http_tests::path_param_echo_handler::PathParamEchoHandler;
use margaret_http_tests::raw_exchange::raw_exchange;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test]
async fn hands_forwarded_path_parameters_to_the_target() {
    let article = Arc::new(PathParamEchoHandler { name: "article" });
    let server = RunningFixtureServer::start_plain_forwarding(
        ["/origin", "/articles/{article}"],
        [
            RouteEntry::new(vec![MethodHandler::head(
                RouteMethod::Get,
                Arc::new(ForwardingHandler {
                    props: || ArticleRouteProps {
                        article: "7".to_string(),
                    },
                    route: ForwardableRoute::new(
                        GetRouteReference::new("http://fixture.test/articles/7".to_string()),
                        FixtureForwardTarget::Article,
                    ),
                }),
            )]),
            RouteEntry::new(vec![MethodHandler::head(RouteMethod::Get, article.clone())]),
        ],
        FixtureForwardTargets {
            article,
            target: Arc::new(StaticHandler {
                body: Vec::new(),
                content_type: "text/plain",
                status: 204,
            }),
        },
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
