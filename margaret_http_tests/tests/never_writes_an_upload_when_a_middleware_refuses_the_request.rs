use std::fs;
use std::sync::Arc;

use tempfile::tempdir;

use margaret_http::body_limit::BodyLimit;
use margaret_http::layer::layer;
use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::raw_exchange::raw_exchange;
use margaret_http_tests::refusing_middleware::RefusingMiddleware;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::uploaded_files_reading_handler::UploadedFilesReadingHandler;
use margaret_http_uploaded_file::upload_config::UploadConfig;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test]
async fn never_writes_an_upload_when_a_middleware_refuses_the_request() {
    let directory = tempdir().expect("a temporary directory");
    let server = RunningFixtureServer::start_plain(
        UploadConfig::enabled(directory.path().to_path_buf()),
        vec![RouteEntry::new(
            "/upload",
            vec![MethodHandler::anonymous(
                RouteMethod::Post,
                layer(
                    Arc::new(RefusingMiddleware),
                    Arc::new(UploadedFilesReadingHandler {
                        limit: BodyLimit::new(1024),
                    }),
                ),
            )],
        )],
    )
    .await;

    let response = raw_exchange(
        server.address(),
        b"POST /upload HTTP/1.1\r\nHost: fixture.test\r\nContent-Type: multipart/form-data; boundary=X\r\nContent-Length: 74\r\nConnection: close\r\n\r\n--X\r\nContent-Disposition: form-data; name=\"f\"; filename=\"a\"\r\n\r\nDATA\r\n--X--\r\n",
    )
    .await;

    assert!(response.starts_with("HTTP/1.1 403 "));
    assert_eq!(
        fs::read_dir(directory.path())
            .expect("the upload directory is readable")
            .count(),
        0
    );

    server.stop().await;
}
