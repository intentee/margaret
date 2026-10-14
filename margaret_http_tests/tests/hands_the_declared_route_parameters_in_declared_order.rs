use std::convert::Infallible;
use std::sync::Arc;

use margaret_http::handler_future::HandlerFuture;
use margaret_http::head_responder::head_responder;
use margaret_http::method_handler::MethodHandler;
use margaret_http::refusal::refusal;
use margaret_http::request::Request;
use margaret_http::route_entry::RouteEntry;
use margaret_http::with_route_parameters::with_route_parameters;
use margaret_http_tests::joined_route_parameters::joined_route_parameters;
use margaret_http_tests::raw_exchange::raw_exchange;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_uploaded_file::upload_config::UploadConfig;
use margaret_route_method::route_method::RouteMethod;

struct TopicArticle;

#[tokio::test]
async fn hands_the_declared_route_parameters_in_declared_order() {
    let handler = head_responder(
        Arc::new(TopicArticle),
        |_responder: Arc<TopicArticle>, request: &Request| -> HandlerFuture<'_, Infallible> {
            Box::pin(with_route_parameters(
                request,
                ["topic", "article"],
                refusal,
                joined_route_parameters,
            ))
        },
    );
    let server = RunningFixtureServer::start_plain(
        UploadConfig::Disabled,
        ["/topics/{topic}/articles/{article}"],
        [RouteEntry::new(vec![MethodHandler::head(
            RouteMethod::Get,
            handler,
        )])],
    )
    .await;

    let response = raw_exchange(
        server.address(),
        b"GET /topics/rust/articles/7 HTTP/1.1\r\nHost: fixture.test\r\nConnection: close\r\n\r\n",
    )
    .await;

    assert!(response.starts_with("HTTP/1.1 200 "));
    assert!(response.ends_with("\r\n\r\nrust/7"));

    server.stop().await;
}
