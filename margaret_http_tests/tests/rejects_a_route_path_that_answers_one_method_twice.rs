use std::sync::Arc;

use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http::router::Router;
use margaret_http::router_error::RouterError;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_route_method::route_method::RouteMethod;

fn not_found() -> Arc<StaticHandler> {
    Arc::new(StaticHandler {
        body: Vec::new(),
        content_type: "text/plain",
        status: 404,
    })
}

#[test]
fn rejects_a_route_path_that_answers_one_method_twice() {
    assert!(matches!(
        Router::build(vec![RouteEntry::new(
            "/items",
            vec![
                MethodHandler::head(RouteMethod::Get, not_found()),
                MethodHandler::head(RouteMethod::Get, not_found()),
            ],
        )]),
        Err(RouterError::DuplicateMethod {
            method: RouteMethod::Get,
            path: "/items",
        })
    ));
}
