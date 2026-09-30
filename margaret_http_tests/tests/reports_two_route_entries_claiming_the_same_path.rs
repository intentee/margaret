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
fn reports_two_route_entries_claiming_the_same_path() {
    assert!(
        ServerRoutes::build(vec![
            RouteEntry::new(
                "/items/{id}",
                vec![MethodHandler::anonymous(RouteMethod::Get, not_found())],
            ),
            RouteEntry::new(
                "/items/{name}",
                vec![MethodHandler::anonymous(RouteMethod::Get, not_found())],
            ),
        ])
        .is_err()
    );
}
