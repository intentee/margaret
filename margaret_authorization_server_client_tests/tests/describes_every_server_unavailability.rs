use headers::Authorization;
use http::HeaderValue;
use http::StatusCode;
use oauth2::basic::BasicTokenType;

use margaret_authorization_server_client::server_endpoint::ServerEndpoint;
use margaret_authorization_server_client::server_unavailability::ServerUnavailability;
use margaret_issuer_request::issuer_exchange_error::IssuerExchangeError;

#[test]
fn describes_every_server_unavailability() {
    let described = [
        ServerUnavailability::AccessTokenUnpresentable {
            source: Authorization::bearer("line\nbreak")
                .expect_err("a line break is not a header value"),
        },
        ServerUnavailability::EmptyAnswer,
        ServerUnavailability::EmptyRefusal {
            status: StatusCode::SERVICE_UNAVAILABLE,
        },
        ServerUnavailability::EndpointUnadvertised {
            endpoint: ServerEndpoint::Authorization,
        },
        ServerUnavailability::EndpointUnadvertised {
            endpoint: ServerEndpoint::Introspection,
        },
        ServerUnavailability::EndpointUnadvertised {
            endpoint: ServerEndpoint::Token,
        },
        ServerUnavailability::EndpointUnadvertised {
            endpoint: ServerEndpoint::Userinfo,
        },
        ServerUnavailability::Exchange(IssuerExchangeError::Oversized { max_bytes: 16 }),
        ServerUnavailability::MalformedAnswer {
            source: serde_path_to_error::deserialize::<_, u8>(
                &mut serde_json::Deserializer::from_str("x"),
            )
            .expect_err("not json"),
        },
        ServerUnavailability::MetadataAwaited,
        ServerUnavailability::UnexpectedContentType {
            content_type: HeaderValue::from_static("text/html"),
        },
        ServerUnavailability::UnsupportedTokenType {
            token_type: BasicTokenType::Extension("dpop".to_string()),
        },
    ]
    .map(|unavailability| unavailability.to_string());

    assert!(described[0].starts_with(
        "the access token issued by the authorization server cannot be presented as a bearer credential: "
    ));
    assert_eq!(
        described[1],
        "the authorization server answered with an empty body"
    );
    assert_eq!(
        described[2],
        "the authorization server refused with status 503 Service Unavailable and an empty body"
    );
    assert_eq!(
        described[3],
        "the authorization server does not advertise its authorization_endpoint"
    );
    assert_eq!(
        described[4],
        "the authorization server does not advertise its introspection_endpoint"
    );
    assert_eq!(
        described[5],
        "the authorization server does not advertise its token_endpoint"
    );
    assert_eq!(
        described[6],
        "the authorization server does not advertise its userinfo_endpoint"
    );
    assert_eq!(
        described[7],
        "the exchange with the authorization server failed: the issuer answered with more than 16 bytes"
    );
    assert!(
        described[8].starts_with("the authorization server answered with a malformed response: ")
    );
    assert_eq!(
        described[9],
        "the metadata of the authorization server has not been discovered yet"
    );
    assert_eq!(
        described[10],
        "the authorization server answered with the content type \"text/html\" instead of application/json"
    );
    assert_eq!(
        described[11],
        "the authorization server issued a token of the unsupported type 'dpop'"
    );
}
