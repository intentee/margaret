use std::sync::Arc;

use margaret_http::body_limit::BodyLimit;
use margaret_http::body_reading::BodyReading;
use margaret_http::content_responder::content_responder;
use margaret_http::handler_future::HandlerFuture;
use margaret_http::method_handler::MethodHandler;
use margaret_http::request::Request;
use margaret_http::request_body::RequestBody;
use margaret_http::request_body_chunk::RequestBodyChunk;
use margaret_http::request_body_stream::RequestBodyStream;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::raw_exchange::raw_exchange;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_uploaded_file::upload_config::UploadConfig;
use margaret_route_method::content_method::ContentMethod;

const ECHOED_BODY: &str = "body";

struct Echo {
    prefix: &'static str,
}

#[tokio::test]
async fn hands_the_request_body_to_a_content_responder() {
    let handler = content_responder(
        Arc::new(Echo { prefix: "echo-" }),
        |responder: Arc<Echo>, _request: &Request, body: RequestBody| -> HandlerFuture<'_> {
            Box::pin(async move {
                let BodyReading::Read(mut stream) =
                    RequestBodyStream::open(body, BodyLimit::new(ECHOED_BODY.len()))
                else {
                    panic!("the body fits its limit");
                };
                let RequestBodyChunk::Data(chunk) = stream.next_chunk().await else {
                    panic!("the body carries data");
                };

                Ok(ResponseContinuation::from(Response::text(
                    200,
                    format!("{}{}", responder.prefix, String::from_utf8_lossy(&chunk)),
                )))
            })
        },
    );
    let server = RunningFixtureServer::start_plain(
        UploadConfig::Disabled,
        vec![RouteEntry::new(
            "/echo",
            vec![MethodHandler::content(ContentMethod::Post, handler)],
        )],
    )
    .await;

    let response = raw_exchange(
        server.address(),
        format!(
            "POST /echo HTTP/1.1\r\nHost: fixture.test\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{ECHOED_BODY}",
            ECHOED_BODY.len()
        )
        .as_bytes(),
    )
    .await;

    assert!(response.ends_with("echo-body"));

    server.stop().await;
}
