use http::Method;

use margaret_http_tests::fixture_request::FixtureRequest;

#[test]
fn carries_the_server_the_request_arrived_at() {
    assert_eq!(
        FixtureRequest::new(Method::GET, "/")
            .into_request()
            .server()
            .address(),
        "127.0.0.1:0"
    );
}
