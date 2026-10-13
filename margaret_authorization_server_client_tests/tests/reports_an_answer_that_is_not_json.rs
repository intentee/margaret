use http::HeaderValue;

use margaret_authorization_server_client::endpoint_outcome::EndpointOutcome;
use margaret_authorization_server_client::server_unavailability::ServerUnavailability;
use margaret_authorization_server_client_tests::answered_token_request::answered_token_request;
use margaret_http_tests::static_handler::StaticHandler;

#[tokio::test]
async fn reports_an_answer_that_is_not_json() {
    assert!(matches!(
        answered_token_request(StaticHandler {
            body: br#"{"access_token":"token","token_type":"bearer"}"#.to_vec(),
            content_type: "text/html",
            status: 200,
        })
        .await,
        EndpointOutcome::Unavailable(ServerUnavailability::UnexpectedContentType { content_type })
            if content_type == HeaderValue::from_static("text/html")
    ));
}
