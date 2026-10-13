use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use http::HeaderValue;
use http::StatusCode;
use oauth2::basic::BasicTokenType;

use margaret_issuer_request::issuer_exchange_error::IssuerExchangeError;

use crate::server_endpoint::ServerEndpoint;

#[derive(Debug)]
pub enum ServerUnavailability {
    AccessTokenUnpresentable {
        source: headers::authorization::InvalidBearerToken,
    },
    EmptyAnswer,
    EmptyRefusal {
        status: StatusCode,
    },
    EndpointUnadvertised {
        endpoint: ServerEndpoint,
    },
    Exchange(IssuerExchangeError),
    MalformedAnswer {
        source: serde_path_to_error::Error<serde_json::Error>,
    },
    MetadataAwaited,
    OversizedAnswer {
        max_bytes: usize,
    },
    UnexpectedContentType {
        content_type: HeaderValue,
    },
    UnsupportedTokenType {
        token_type: BasicTokenType,
    },
}

impl Display for ServerUnavailability {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::AccessTokenUnpresentable { source } => write!(
                formatter,
                "the access token issued by the authorization server cannot be presented as a bearer credential: {source}"
            ),
            Self::EmptyAnswer => {
                formatter.write_str("the authorization server answered with an empty body")
            }
            Self::EmptyRefusal { status } => write!(
                formatter,
                "the authorization server refused with status {status} and an empty body"
            ),
            Self::EndpointUnadvertised { endpoint } => write!(
                formatter,
                "the authorization server does not advertise its {endpoint}"
            ),
            Self::Exchange(failure) => write!(
                formatter,
                "the exchange with the authorization server failed: {failure}"
            ),
            Self::MalformedAnswer { source } => write!(
                formatter,
                "the authorization server answered with a malformed response: {source}"
            ),
            Self::MetadataAwaited => formatter
                .write_str("the metadata of the authorization server has not been discovered yet"),
            Self::OversizedAnswer { max_bytes } => write!(
                formatter,
                "the authorization server answered with more than {max_bytes} bytes"
            ),
            Self::UnexpectedContentType { content_type } => write!(
                formatter,
                "the authorization server answered with the content type {content_type:?} instead of application/json"
            ),
            Self::UnsupportedTokenType { token_type } => write!(
                formatter,
                "the authorization server issued a token of the unsupported type '{}'",
                token_type.as_ref()
            ),
        }
    }
}
