use margaret_http::matchit::InsertError;
use margaret_http::router_error::RouterError;
use margaret_http::server_paths::ServerPaths;

#[test]
fn reports_two_route_entries_claiming_the_same_path() {
    assert!(matches!(
        ServerPaths::build(["/items/{id}", "/items/{name}"]),
        Err(RouterError::Path(InsertError::Conflict { .. }))
    ));
}
