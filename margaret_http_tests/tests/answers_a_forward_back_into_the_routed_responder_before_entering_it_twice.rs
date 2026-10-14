use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use async_trait::async_trait;

use margaret_handler_error::handler_error::HandlerError;
use margaret_http::forwardable_route::ForwardableRoute;
use margaret_http::get_route_reference::GetRouteReference;
use margaret_http::head_handler::HeadHandler;
use margaret_http::method_handler::MethodHandler;
use margaret_http::request::Request;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::fixture_forward_target::FixtureForwardTarget;
use margaret_http_tests::fixture_forward_targets::FixtureForwardTargets;
use margaret_http_tests::path_param_echo_handler::PathParamEchoHandler;
use margaret_http_tests::raw_exchange::raw_exchange;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::target_route_props::TargetRouteProps;
use margaret_route_method::route_method::RouteMethod;

struct CountedSelfForward {
    entries: AtomicUsize,
    route: ForwardableRoute<FixtureForwardTarget, TargetRouteProps, GetRouteReference>,
}

#[async_trait]
impl HeadHandler<FixtureForwardTarget> for CountedSelfForward {
    async fn handle(
        &self,
        _request: &Request,
    ) -> Result<ResponseContinuation<FixtureForwardTarget>, HandlerError> {
        self.entries.fetch_add(1, Ordering::SeqCst);

        Ok(ResponseContinuation::from(
            self.route.forward_to(TargetRouteProps),
        ))
    }
}

#[tokio::test]
async fn answers_a_forward_back_into_the_routed_responder_before_entering_it_twice() {
    let origin = Arc::new(CountedSelfForward {
        entries: AtomicUsize::new(0),
        route: ForwardableRoute::new(
            GetRouteReference::new("http://fixture.test/target".to_string()),
            FixtureForwardTarget::Target,
        ),
    });
    let server = RunningFixtureServer::start_plain_forwarding(
        ["/target"],
        [RouteEntry::new(vec![MethodHandler::head(
            RouteMethod::Get,
            origin.clone(),
        )])],
        FixtureForwardTargets {
            article: Arc::new(PathParamEchoHandler { name: "article" }),
            target: origin.clone(),
        },
    )
    .await;

    let response = raw_exchange(
        server.address(),
        b"GET /target HTTP/1.1\r\nHost: fixture.test\r\nConnection: close\r\n\r\n",
    )
    .await;

    server.stop().await;

    assert!(response.starts_with("HTTP/1.1 500 "));
    assert_eq!(origin.entries.load(Ordering::SeqCst), 1);
}
