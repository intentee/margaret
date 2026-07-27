use margaret_http::http::HeaderValue;
use margaret_http::http::Request;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;

#[must_use]
pub fn web_socket_client_request(url: String) -> Request<()> {
    let mut request = url
        .into_client_request()
        .expect("the test WebSocket URL is valid");
    request
        .headers_mut()
        .insert("origin", HeaderValue::from_static("https://example.test"));

    request
}
