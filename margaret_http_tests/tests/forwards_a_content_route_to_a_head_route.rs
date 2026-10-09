use std::collections::HashMap;
use std::sync::Arc;

use margaret_http::body_limit::BodyLimit;
use margaret_http::body_reading::BodyReading;
use margaret_http::content_responder::content_responder;
use margaret_http::forward::Forward;
use margaret_http::handler_future::HandlerFuture;
use margaret_http::method_handler::MethodHandler;
use margaret_http::request::Request;
use margaret_http::request_body::RequestBody;
use margaret_http::request_body_chunk::RequestBodyChunk;
use margaret_http::request_body_stream::RequestBodyStream;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::raw_exchange::raw_exchange;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_http_uploaded_file::upload_config::UploadConfig;
use margaret_route_method::content_method::ContentMethod;

const SUBMITTED_BODY: &str = "note";

struct Submission;

#[tokio::test]
async fn forwards_a_content_route_to_a_head_route() {
    let submit = content_responder(
        Arc::new(Submission),
        |_responder: Arc<Submission>, _request: &Request, body: RequestBody| -> HandlerFuture<'_> {
            Box::pin(async move {
                let BodyReading::Read(mut stream) =
                    RequestBodyStream::open(body, BodyLimit::new(SUBMITTED_BODY.len()))
                else {
                    panic!("the body fits its limit");
                };
                let RequestBodyChunk::Data(_) = stream.next_chunk().await else {
                    panic!("the body carries data");
                };

                Ok(ResponseContinuation::from(Forward::new(
                    "confirmation",
                    HashMap::new(),
                )))
            })
        },
    );
    let server = RunningFixtureServer::start_plain(
        UploadConfig::Disabled,
        vec![
            RouteEntry::new(
                "/submissions",
                vec![MethodHandler::content(ContentMethod::Post, submit)],
            ),
            RouteEntry::new(
                "/confirmation",
                vec![MethodHandler::forwardable(
                    "confirmation",
                    Arc::new(StaticHandler {
                        body: b"confirmed".to_vec(),
                        content_type: "text/plain",
                        status: 201,
                    }),
                )],
            ),
        ],
    )
    .await;

    let response = raw_exchange(
        server.address(),
        format!(
            "POST /submissions HTTP/1.1\r\nHost: fixture.test\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{SUBMITTED_BODY}",
            SUBMITTED_BODY.len()
        )
        .as_bytes(),
    )
    .await;

    assert!(response.starts_with("HTTP/1.1 201"));
    assert!(response.ends_with("confirmed"));

    server.stop().await;
}
