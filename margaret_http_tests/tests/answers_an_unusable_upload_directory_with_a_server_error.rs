use std::sync::Arc;

use margaret_http::body_limit::BodyLimit;
use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::raw_exchange::raw_exchange;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::uploaded_files_reading_handler::UploadedFilesReadingHandler;
use margaret_http_uploaded_file::upload_config::UploadConfig;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test]
async fn answers_an_unusable_upload_directory_with_a_server_error() {
    let server = RunningFixtureServer::start_plain(
        UploadConfig::enabled("/margaret-nonexistent-upload-directory".into()),
        vec![RouteEntry::new(
            "/upload",
            vec![MethodHandler::anonymous(
                RouteMethod::Post,
                Arc::new(UploadedFilesReadingHandler {
                    limit: BodyLimit::new(1024),
                }),
            )],
        )],
    )
    .await;

    let response = raw_exchange(server.address(), b"POST /upload HTTP/1.1\r\nHost: fixture.test\r\nContent-Type: multipart/form-data; boundary=X\r\nContent-Length: 74\r\nConnection: close\r\n\r\n--X\r\nContent-Disposition: form-data; name=\"f\"; filename=\"a\"\r\n\r\nDATA\r\n--X--\r\n").await;

    assert!(response.starts_with("HTTP/1.1 500 "));

    server.stop().await;
}
