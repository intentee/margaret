use std::sync::Arc;

use reqwest::Client;
use url::Url;

use margaret_endpoint::static_endpoint::StaticEndpoint;
use margaret_jwks_client::jwks_client_error::JwksClientError;
use margaret_jwks_client::public_jwks_holder::PublicJwksHolder;
use margaret_jwks_client::public_jwks_poll_service::PublicJwksPollService;

#[tokio::test]
async fn public_jwks_poll_service_rejects_an_issuer_that_cannot_be_a_base() {
    let issuer = Url::parse("mailto:issuer@example.org").expect("the mailto url parses");
    let service = PublicJwksPollService {
        endpoint_provider: Arc::new(StaticEndpoint::new(issuer)),
        http_client: Client::new(),
        public_jwks_holder: PublicJwksHolder::default(),
    };

    let error = service
        .fetch_public_jwks()
        .await
        .expect_err("an issuer url that cannot be a base has no well known jwks path");

    assert!(matches!(error, JwksClientError::IssuerUrlNotABase { .. }));
    assert_eq!(
        error.to_string(),
        "the issuer url 'mailto:issuer@example.org' cannot carry the well known jwks path: relative URL with a cannot-be-a-base base"
    );
}
