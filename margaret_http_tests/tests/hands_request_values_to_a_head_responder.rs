use std::sync::Arc;

use margaret_http::handler_future::HandlerFuture;
use margaret_http::head_responder::head_responder;
use margaret_http::method_handler::MethodHandler;
use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::raw_exchange::raw_exchange;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_uploaded_file::upload_config::UploadConfig;
use margaret_route_method::route_method::RouteMethod;

struct Echo {
    prefix: &'static str,
}

#[tokio::test]
async fn hands_request_values_to_a_head_responder() {
    let handler = head_responder(
        Arc::new(Echo { prefix: "echo-" }),
        |responder: Arc<Echo>, request: &Request| -> HandlerFuture<'_> {
            Box::pin(async move {
                Ok(ResponseContinuation::from(Response::text(
                    200,
                    format!(
                        "{}{}",
                        responder.prefix,
                        request.path_param("id").unwrap_or("absent")
                    ),
                )))
            })
        },
    );
    let server = RunningFixtureServer::start_plain(
        UploadConfig::Disabled,
        vec![RouteEntry::new(
            "/echo/{id}",
            vec![MethodHandler::head(RouteMethod::Get, handler)],
        )],
    )
    .await;

    let response = raw_exchange(
        server.address(),
        b"GET /echo/7 HTTP/1.1\r\nHost: fixture.test\r\nConnection: close\r\n\r\n",
    )
    .await;

    assert!(response.ends_with("echo-7"));

    server.stop().await;
}
