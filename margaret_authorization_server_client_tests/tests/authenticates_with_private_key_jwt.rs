use std::collections::BTreeSet;
use std::sync::Arc;

use oauth2::AuthorizationCode;
use oauth2::EmptyExtraTokenFields;
use oauth2::ExtraTokenFields;
use oauth2::RedirectUrl;
use oauth2::StandardTokenIntrospectionResponse;
use oauth2::StandardTokenResponse;
use oauth2::TokenResponse;
use oauth2::basic::BasicTokenType;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use url::Url;

use margaret_authorization_server_client::endpoint_outcome::EndpointOutcome;
use margaret_authorization_server_client::form_parameter::FormParameter;
use margaret_authorization_server_client::target_audience::TargetAudience;
use margaret_authorization_server_client::token_target::TokenTarget;
use margaret_authorization_server_client_tests::fixture_authorization_server::FixtureAuthorizationServer;
use margaret_authorization_server_client_tests::rfc_7636_verifier::rfc_7636_verifier;
use margaret_http::body_limit::BodyLimit;
use margaret_http::method_handler::MethodHandler;
use margaret_http_tests::echo_wrapping::EchoWrapping;
use margaret_http_tests::form_echo_handler::FormEchoHandler;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_secret_store_tests::rolled_roller::rolled_roller;
use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_jws_verification::header_type::HeaderType;
use margaret_jws_verification::jws_verification::JwsVerification;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_oauth_client::client_authentication::ClientAuthentication;
use margaret_oauth_vocabulary::grant_type::GrantType;
use margaret_route_method::content_method::ContentMethod;

#[derive(Debug, Deserialize, Serialize)]
struct EchoFields {
    echo: Value,
}

impl ExtraTokenFields for EchoFields {}

async fn echo_server(path: &'static str, wrapping: EchoWrapping) -> FixtureAuthorizationServer {
    FixtureAuthorizationServer::start(
        path,
        MethodHandler::content(
            ContentMethod::Post,
            Arc::new(FormEchoHandler {
                limit: BodyLimit::new(4096),
                wrapping,
            }),
        ),
    )
    .await
}

fn echoed_token<TFields: ExtraTokenFields>(
    outcome: EndpointOutcome<StandardTokenResponse<TFields, BasicTokenType>>,
) -> Value {
    let EndpointOutcome::Answered(response) = outcome else {
        panic!("the token endpoint answers");
    };

    serde_json::from_str(response.access_token().secret()).expect("the echo is json")
}

fn authenticated_claims(echoed: &Value, key_set: &VerificationKeySet) -> Value {
    let fields = &echoed["fields"];
    let CompactJwsParsing::Parsed(jws) = CompactJws::parse(
        fields["client_assertion"]
            .as_str()
            .expect("the client assertion is posted"),
    ) else {
        panic!("the client assertion is a compact jws");
    };

    assert_eq!(echoed["authorization"], Value::Null);
    assert_eq!(
        fields["client_assertion_type"],
        "urn:ietf:params:oauth:client-assertion-type:jwt-bearer"
    );
    assert_eq!(fields["client_id"], "client:id");
    assert_eq!(
        jws.typ(),
        Some(&HeaderType::Supported(JwtType::ClientAuthentication))
    );
    assert!(matches!(key_set.verify(&jws), JwsVerification::Verified(_)));

    let claims: Value =
        serde_json::from_slice(jws.payload()).expect("the client assertion claims are json");

    assert_eq!(claims["sub"], "client:id");

    claims
}

#[tokio::test]
async fn authenticates_with_private_key_jwt() {
    let secret = fresh_secret(SigningCurve::P256);
    let key_set = secret.published_key_set().clone();
    let server = echo_server("/token", EchoWrapping::AccessToken).await;

    let outcome = server
        .client(ClientAuthentication::PrivateKeyJwt(
            rolled_roller(secret).await,
        ))
        .client_credentials(&TokenTarget {
            audience: TargetAudience::Unspecified,
            scopes: BTreeSet::new(),
        })
        .await;

    server.stop().await;

    let echoed = echoed_token(outcome);
    let claims = authenticated_claims(&echoed, &key_set);

    assert_eq!(echoed["fields"]["grant_type"], "client_credentials");
    assert_eq!(claims["aud"], "https://localhost");
    assert_eq!(claims["iss"], "client:id");
    assert_eq!(
        claims["exp"].as_i64().expect("the assertion expires")
            - claims["iat"].as_i64().expect("the assertion is issued"),
        30
    );
    assert!(claims["jti"].is_string());
}

#[tokio::test]
async fn authenticates_a_grant_with_private_key_jwt() {
    let secret = fresh_secret(SigningCurve::P256);
    let key_set = secret.published_key_set().clone();
    let server = echo_server("/token", EchoWrapping::AccessToken).await;

    let outcome = server
        .client(ClientAuthentication::PrivateKeyJwt(
            rolled_roller(secret).await,
        ))
        .request_grant::<EmptyExtraTokenFields>(
            GrantType::TokenExchange,
            vec![FormParameter {
                name: "subject_token",
                value: "grant.subject.token".to_string(),
            }],
        )
        .await;

    server.stop().await;

    let echoed = echoed_token(outcome);

    authenticated_claims(&echoed, &key_set);

    assert_eq!(echoed["fields"]["subject_token"], "grant.subject.token");
    assert_eq!(
        echoed["fields"]["grant_type"],
        "urn:ietf:params:oauth:grant-type:token-exchange"
    );
}

#[tokio::test]
async fn authenticates_a_code_exchange_with_private_key_jwt() {
    let secret = fresh_secret(SigningCurve::P256);
    let key_set = secret.published_key_set().clone();
    let server = echo_server("/token", EchoWrapping::AccessToken).await;

    let outcome = server
        .client(ClientAuthentication::PrivateKeyJwt(
            rolled_roller(secret).await,
        ))
        .exchange_authorization_code::<EmptyExtraTokenFields>(
            AuthorizationCode::new("SplxlOBeZQQYbYS6WxSbIA".to_string()),
            &rfc_7636_verifier(),
            RedirectUrl::from_url(
                Url::parse("https://client.example/callback").expect("the callback is a url"),
            ),
        )
        .await;

    server.stop().await;

    let echoed = echoed_token(outcome);

    authenticated_claims(&echoed, &key_set);

    assert_eq!(echoed["fields"]["code"], "SplxlOBeZQQYbYS6WxSbIA");
}

#[tokio::test]
async fn authenticates_an_introspection_with_private_key_jwt() {
    let secret = fresh_secret(SigningCurve::P256);
    let key_set = secret.published_key_set().clone();
    let server = echo_server("/introspect", EchoWrapping::ActiveIntrospection).await;

    let outcome = server
        .client(ClientAuthentication::PrivateKeyJwt(
            rolled_roller(secret).await,
        ))
        .introspect::<EchoFields>("opaque-token")
        .await;

    server.stop().await;

    let EndpointOutcome::Answered(introspection): EndpointOutcome<
        StandardTokenIntrospectionResponse<EchoFields, BasicTokenType>,
    > = outcome
    else {
        panic!("the introspection endpoint answers");
    };
    let echoed = &introspection.extra_fields().echo;

    authenticated_claims(echoed, &key_set);

    assert_eq!(echoed["fields"]["token"], "opaque-token");
    assert_eq!(echoed["fields"]["token_type_hint"], "access_token");
}
