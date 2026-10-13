use reqwest::Client;
use reqwest::Method;
use reqwest::Request;
use url::Url;

use margaret_issuer_request::issuer_exchange_error::IssuerExchangeError;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;

#[tokio::test]
async fn refuses_a_plaintext_url_through_a_preconfigured_client_so_secrets_stay_encrypted() {
    let client = IssuerRequestClient::preconfigured(Client::builder())
        .expect("the preconfigured client builds");
    let request = Request::new(
        Method::POST,
        Url::parse("http://localhost/sessions/refresh").expect("the fixture url parses"),
    );

    assert!(matches!(
        client.exchange(request).await,
        Err(IssuerExchangeError::Transport(error)) if error.is_builder()
    ));
}
