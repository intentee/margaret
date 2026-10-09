use std::sync::Arc;

use cookie::Cookie;

use margaret_http::handler_future::HandlerFuture;
use margaret_http::head_responder::head_responder;
use margaret_http::method_handler::MethodHandler;
use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::cookie_changing_forward_handler::CookieChangingForwardHandler;
use margaret_http_tests::raw_exchange::raw_exchange;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_uploaded_file::upload_config::UploadConfig;
use margaret_route_method::route_method::RouteMethod;

struct Starting;

#[tokio::test]
async fn keeps_the_cookies_a_forward_target_sets_after_the_carried_ones() {
    let starting = head_responder(
        Arc::new(Starting),
        |_responder: Arc<Starting>, _request: &Request| -> HandlerFuture<'_> {
            Box::pin(async {
                Ok(ResponseContinuation::from(
                    Response::text(200, "started").set_cookie(&Cookie::new("session", "started")),
                ))
            })
        },
    );
    let server = RunningFixtureServer::start_plain(
        UploadConfig::Disabled,
        vec![
            RouteEntry::new(
                "/forwarding",
                vec![MethodHandler::head(
                    RouteMethod::Get,
                    Arc::new(CookieChangingForwardHandler {
                        cookie: Cookie::new("session", "removed"),
                        target: "starting",
                    }),
                )],
            ),
            RouteEntry::new(
                "/starting",
                vec![MethodHandler::forwardable("starting", starting)],
            ),
        ],
    )
    .await;

    let response = raw_exchange(
        server.address(),
        b"GET /forwarding HTTP/1.1\r\nHost: fixture.test\r\nConnection: close\r\n\r\n",
    )
    .await;

    server.stop().await;

    assert!(
        response
            .lines()
            .filter(|line| line.starts_with("set-cookie:"))
            .eq(["set-cookie: session=removed", "set-cookie: session=started"])
    );
}
