use margaret_issuer_request::issuer_exchange_error::IssuerExchangeError;

#[test]
fn describes_every_exchange_error() {
    assert!(
        IssuerExchangeError::Transport(
            reqwest::Client::new()
                .get("not a url")
                .build()
                .expect_err("a relative url is not requestable"),
        )
        .to_string()
        .starts_with("the issuer could not be reached: ")
    );
}
