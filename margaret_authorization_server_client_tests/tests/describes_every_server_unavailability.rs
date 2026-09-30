use headers::Authorization;
use oauth2::basic::BasicTokenType;

use margaret_authorization_server_client::server_endpoint::ServerEndpoint;
use margaret_authorization_server_client::server_unavailability::ServerUnavailability;

#[test]
fn describes_every_server_unavailability() {
    let described = [
        ServerUnavailability::AccessTokenUnpresentable {
            source: Authorization::bearer("line\nbreak")
                .expect_err("a line break is not a header value"),
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
        ServerUnavailability::MalformedAnswer {
            source: serde_path_to_error::deserialize::<_, u8>(
                &mut serde_json::Deserializer::from_str("x"),
            )
            .expect_err("not json"),
        },
        ServerUnavailability::MetadataAwaited,
        ServerUnavailability::Oversized { max_bytes: 16 },
        ServerUnavailability::Transport(
            reqwest::Client::new()
                .get("not a url")
                .build()
                .expect_err("a relative url is not requestable"),
        ),
        ServerUnavailability::UnexpectedAnswer {
            description: "server returned empty error response".to_string(),
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
        "the authorization server does not advertise its authorization_endpoint"
    );
    assert_eq!(
        described[2],
        "the authorization server does not advertise its introspection_endpoint"
    );
    assert_eq!(
        described[3],
        "the authorization server does not advertise its token_endpoint"
    );
    assert_eq!(
        described[4],
        "the authorization server does not advertise its userinfo_endpoint"
    );
    assert!(
        described[5].starts_with("the authorization server answered with a malformed response: ")
    );
    assert_eq!(
        described[6],
        "the metadata of the authorization server has not been discovered yet"
    );
    assert_eq!(
        described[7],
        "the authorization server answered with more than 16 bytes"
    );
    assert!(described[8].starts_with("the authorization server could not be reached: "));
    assert_eq!(
        described[9],
        "the authorization server answered unexpectedly: server returned empty error response"
    );
    assert_eq!(
        described[10],
        "the authorization server issued a token of the unsupported type 'dpop'"
    );
}
