use oauth2::AsyncHttpClient;

use margaret_issuer_request::issuer_exchange_error::IssuerExchangeError;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;

#[tokio::test]
async fn refuses_to_exchange_without_an_absolute_url() {
    let client = IssuerRequestClient::create().expect("the issuer request client builds");
    let request = http::Request::builder()
        .method(http::Method::POST)
        .uri("/token")
        .body(Vec::new())
        .expect("the fixture request is well formed");

    assert!(matches!(
        client.call(request).await,
        Err(IssuerExchangeError::Transport(error)) if error.is_builder()
    ));
}
