use std::sync::Arc;

use reqwest::Client;
use url::Url;

use margaret_jwks_client::jwk_public_set_holder::JwkPublicSetHolder;
use margaret_jwks_client::jwk_public_set_poll_service::JwkPublicSetPollService;
use margaret_jwks_client::jwks_client_error::JwksClientError;
use margaret_jwks_client_tests::static_endpoint::StaticEndpoint;

#[tokio::test]
async fn jwk_public_set_poll_service_reports_an_endpoint_that_cannot_be_a_base() {
    let service = JwkPublicSetPollService {
        endpoint: Arc::new(StaticEndpoint::new(
            Url::parse("mailto:issuer@example.org").expect("the fixture url parses"),
        )),
        http_client: Client::new(),
        jwk_public_set_holder: JwkPublicSetHolder::default(),
    };

    let Err(error) = service.poll().await else {
        panic!("an endpoint that cannot be a base has no well known jwks path");
    };

    assert!(matches!(error, JwksClientError::IssuerUrlNotABase { .. }));
    assert_eq!(
        error.to_string(),
        "the issuer url 'mailto:issuer@example.org' cannot carry the well known jwks path: relative URL with a cannot-be-a-base base"
    );
}
