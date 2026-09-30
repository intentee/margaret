use std::sync::Arc;

use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http::server_routes::ServerRoutes;
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
fn collects_only_the_forwardable_handlers_of_a_server() {
    let server_routes = ServerRoutes::build(vec![RouteEntry::new(
        "/articles",
        vec![
            MethodHandler::forwardable("get_articles", not_found()),
            MethodHandler::anonymous(RouteMethod::Post, not_found()),
        ],
    )])
    .expect("the entries build a router");

    assert_eq!(server_routes.named_handlers.len(), 1);
}
