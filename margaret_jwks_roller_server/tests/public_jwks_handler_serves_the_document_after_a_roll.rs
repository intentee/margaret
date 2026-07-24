use http::Method;

use margaret_http::handler::Handler as _;
use margaret_http::request::Request;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_jwks_roller::memory_jwks_secret_storage::MemoryJwksSecretStorage;
use margaret_jwks_roller::well_known_jwks_path::WELL_KNOWN_JWKS_PATH;
use margaret_jwks_roller_server::jwks_publication::JwksPublication;
use margaret_jwks_roller_server::roll_and_publish::roll_and_publish;

#[tokio::test]
async fn public_jwks_handler_serves_the_document_after_a_roll() {
    let publication = JwksPublication::new();

    roll_and_publish(
        &MemoryJwksSecretStorage,
        &publication.jwks_secret_holder(),
        &publication.jwks_document_holder(),
    )
    .expect("the first roll publishes the document");

    let handler = publication.public_jwks_handler();
    let request = Request::new(Method::GET, WELL_KNOWN_JWKS_PATH.to_string());

    let ResponseContinuation::Done(response) = handler.handle(&request).await else {
        panic!("the jwks handler always responds directly");
    };

    assert_eq!(response.status(), 200);
}
