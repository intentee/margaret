use std::sync::Arc;

use http::Method;
use tokio_util::sync::CancellationToken;

use margaret_http::handler::Handler as _;
use margaret_http::request::Request;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_jwks_roller::memory_jwks_secret_storage::MemoryJwksSecretStorage;
use margaret_jwks_roller::well_known_jwks_path::WELL_KNOWN_JWKS_PATH;
use margaret_jwks_roller_server::JwksRollerServerBundle;
use margaret_jwks_roller_server::JwksRollerServerBundleParams;

#[tokio::test]
async fn public_jwks_handler_reports_not_ready_before_the_first_roll() {
    let bundle = JwksRollerServerBundle::new(JwksRollerServerBundleParams {
        storage: Arc::new(MemoryJwksSecretStorage),
    });
    let handler = bundle.public_jwks_handler();
    let request = Request::new(
        Method::GET,
        WELL_KNOWN_JWKS_PATH.to_string(),
        CancellationToken::new(),
    );

    let Ok(ResponseContinuation::Done(response)) = handler.handle(&request).await else {
        panic!("the jwks handler always responds directly");
    };

    assert_eq!(response.status(), 503);
}
