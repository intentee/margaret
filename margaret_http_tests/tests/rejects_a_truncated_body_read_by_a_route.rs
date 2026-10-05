use std::sync::Arc;

use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use margaret_http::body_limit::BodyLimit;
use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::json_reading_handler::JsonReadingHandler;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_uploaded_file::upload_config::UploadConfig;
use margaret_route_method::content_method::ContentMethod;

#[tokio::test]
async fn rejects_a_truncated_body_read_by_a_route() {
    let server = RunningFixtureServer::start_plain(
        UploadConfig::Disabled,
        vec![RouteEntry::new(
            "/json",
            vec![MethodHandler::content(
                ContentMethod::Post,
                Arc::new(JsonReadingHandler {
                    limit: BodyLimit::new(1024),
                }),
            )],
        )],
    )
    .await;
    let mut stream = TcpStream::connect(server.address())
        .await
        .expect("the client connects");

    stream
        .write_all(b"POST /json HTTP/1.1\r\nHost: fixture.test\r\nContent-Type: application/json\r\nContent-Length: 64\r\nConnection: close\r\n\r\nshort")
        .await
        .expect("the truncated request is sent");
    stream.shutdown().await.expect("the write half closes");

    let mut response = Vec::new();

    stream
        .read_to_end(&mut response)
        .await
        .expect("the response is read");

    assert!(String::from_utf8_lossy(&response).starts_with("HTTP/1.1 400 "));

    server.stop().await;
}
