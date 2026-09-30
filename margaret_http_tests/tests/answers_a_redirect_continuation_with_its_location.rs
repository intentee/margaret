use std::sync::Arc;

use margaret_http::forwardable_route::ForwardableRoute;
use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http::url_segment::UrlSegment;
use margaret_http_tests::raw_exchange::raw_exchange;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::see_other_handler::SeeOtherHandler;
use margaret_http_uploaded_file::upload_config::UploadConfig;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test]
async fn answers_a_redirect_continuation_with_its_location() {
    let server = RunningFixtureServer::start_plain(
        UploadConfig::Disabled,
        vec![RouteEntry::new(
            "/moved",
            vec![MethodHandler::anonymous(
                RouteMethod::Get,
                Arc::new(SeeOtherHandler {
                    route: ForwardableRoute::new(
                        Arc::from("http://fixture.test"),
                        vec![UrlSegment::Literal("/greeting")],
                    ),
                }),
            )],
        )],
    )
    .await;

    let response = raw_exchange(
        server.address(),
        b"GET /moved HTTP/1.1\r\nHost: fixture.test\r\nConnection: close\r\n\r\n",
    )
    .await;

    assert!(response.starts_with("HTTP/1.1 303 "));
    assert!(response.contains("location: http://fixture.test/greeting\r\n"));

    server.stop().await;
}
