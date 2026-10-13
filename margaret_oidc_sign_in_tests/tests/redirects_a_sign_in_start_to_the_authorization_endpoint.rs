use margaret_authorization_server_client_tests::secret_basic_authentication::secret_basic_authentication;
use margaret_http::head_handler::HeadHandler;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_http_tests::fixture_request::FixtureRequest;
use margaret_oidc_sign_in::sign_in_start_handler::SignInStartHandler;
use margaret_oidc_sign_in_tests::begun_sign_in::BegunSignIn;
use margaret_oidc_sign_in_tests::sign_in_fixture::SignInFixture;

#[tokio::test]
async fn redirects_a_sign_in_start_to_the_authorization_endpoint() {
    let fixture = SignInFixture::start(secret_basic_authentication()).await;
    let answered = SignInStartHandler::create(fixture.flow.clone())
        .handle(&FixtureRequest::new(http::Method::GET, "/sign-in").into_request())
        .await;

    fixture.server.stop().await;

    let Ok(ResponseContinuation::Done(response)) = answered else {
        panic!("the sign-in start is answered");
    };

    assert_eq!(response.status(), 303);
    assert_eq!(
        BegunSignIn::of(&response).authorization_parameter("client_id"),
        "client:id"
    );
}
