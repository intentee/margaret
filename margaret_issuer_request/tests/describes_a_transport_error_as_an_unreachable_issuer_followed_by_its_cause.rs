use margaret_issuer_request::issuer_exchange_error::IssuerExchangeError;

#[test]
fn describes_a_transport_error_as_an_unreachable_issuer_followed_by_its_cause() {
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
