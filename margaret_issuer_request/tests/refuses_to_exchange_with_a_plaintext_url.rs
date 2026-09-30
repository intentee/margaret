use reqwest::Method;
use reqwest::Request;
use url::Url;

use margaret_issuer_request::issuer_exchange_error::IssuerExchangeError;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;

#[tokio::test]
async fn refuses_to_exchange_with_a_plaintext_url() {
    let client = IssuerRequestClient::create().expect("the issuer request client builds");
    let request = Request::new(
        Method::POST,
        Url::parse("http://localhost/token").expect("the fixture url parses"),
    );

    assert!(matches!(
        client.exchange(request).await,
        Err(IssuerExchangeError::Transport(error)) if error.is_builder()
    ));
}
