use std::io;
use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;
use futures_util::StreamExt;
use futures_util::stream;
use tokio_util::sync::CancellationToken;

use margaret_handler_error::handler_error::HandlerError;
use margaret_http::head_handler::HeadHandler;
use margaret_http::method_handler::MethodHandler;
use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::streamed_exchange::StreamedExchange;
use margaret_http_uploaded_file::upload_config::UploadConfig;
use margaret_route_method::route_method::RouteMethod;

struct FailingDownload {
    gate: CancellationToken,
}

#[async_trait]
impl HeadHandler for FailingDownload {
    async fn handle(&self, _request: &Request) -> Result<ResponseContinuation, HandlerError> {
        let gate = self.gate.clone();
        let chunks =
            stream::iter([Ok(Bytes::from_static(b"partial"))]).chain(stream::once(async move {
                gate.cancelled().await;

                Err(io::Error::other("the artifact store went away"))
            }));

        Ok(ResponseContinuation::Done(Response::stream(
            200,
            "application/octet-stream",
            chunks,
        )))
    }
}

#[tokio::test]
async fn aborts_a_response_whose_body_stream_fails() {
    let gate = CancellationToken::new();
    let server = RunningFixtureServer::start_plain(
        UploadConfig::Disabled,
        vec![RouteEntry::new(
            "/download",
            vec![MethodHandler::head(
                RouteMethod::Get,
                Arc::new(FailingDownload { gate: gate.clone() }),
            )],
        )],
    )
    .await;
    let mut exchange = StreamedExchange::start(
        server.address(),
        b"GET /download HTTP/1.1\r\nHost: fixture.test\r\nConnection: close\r\n\r\n",
    )
    .await;

    exchange.receive_until("partial").await;
    gate.cancel();

    let response = exchange.finish().await;

    assert!(!response.ends_with("0\r\n\r\n"));

    server.stop().await;
}
