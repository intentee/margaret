use std::convert::Infallible;
use std::sync::Arc;

use tokio_util::sync::CancellationToken;
use trzcina::Service;

use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http::server_paths::ServerPaths;
use margaret_http::transport_config::TransportConfig;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_http_uploaded_file::upload_config::UploadConfig;
use margaret_route_method::route_method::RouteMethod;
use margaret_service::server_listener::ServerListener;
use margaret_service::server_service::ServerService;

fn listener() -> ServerListener {
    ServerListener {
        address: "127.0.0.1:0".to_string(),
        transport: TransportConfig::Plain,
        uploads: UploadConfig::Disabled,
    }
}

fn not_found() -> Arc<StaticHandler> {
    Arc::new(StaticHandler {
        body: Vec::new(),
        content_type: "text/plain",
        status: 404,
    })
}

#[tokio::test]
async fn binds_and_drains_on_cancellation_so_shutdown_finishes_without_error() {
    let cancellation_token = CancellationToken::new();

    cancellation_token.cancel();

    Box::new(ServerService::new(
        listener(),
        ServerPaths::build([])
            .expect("an empty path set builds")
            .routes::<Infallible>([], ()),
    ))
    .run(cancellation_token)
    .await
    .expect("the server binds and drains");
}

#[tokio::test]
async fn fails_to_start_when_a_route_path_answers_one_method_twice() {
    let outcome = Box::new(ServerService::new(
        listener(),
        ServerPaths::build(["/items"])
            .expect("the route paths do not conflict")
            .routes::<Infallible>(
                [RouteEntry::new(vec![
                    MethodHandler::head(RouteMethod::Get, not_found()),
                    MethodHandler::head(RouteMethod::Get, not_found()),
                ])],
                (),
            ),
    ))
    .run(CancellationToken::new())
    .await;

    assert!(outcome.is_err());
}
