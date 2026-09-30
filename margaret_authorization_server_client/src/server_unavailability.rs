use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use oauth2::basic::BasicTokenType;

use margaret_issuer_request::issuer_exchange_error::IssuerExchangeError;

use crate::server_endpoint::ServerEndpoint;

#[derive(Debug)]
pub enum ServerUnavailability {
    AccessTokenUnpresentable {
        source: headers::authorization::InvalidBearerToken,
    },
    EndpointUnadvertised {
        endpoint: ServerEndpoint,
    },
    MalformedAnswer {
        source: serde_path_to_error::Error<serde_json::Error>,
    },
    MetadataAwaited,
    Oversized {
        max_bytes: usize,
    },
    Transport(reqwest::Error),
    UnexpectedAnswer {
        description: String,
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
            Self::EndpointUnadvertised { endpoint } => write!(
                formatter,
                "the authorization server does not advertise its {endpoint}"
            ),
            Self::MalformedAnswer { source } => write!(
                formatter,
                "the authorization server answered with a malformed response: {source}"
            ),
            Self::MetadataAwaited => formatter
                .write_str("the metadata of the authorization server has not been discovered yet"),
            Self::Oversized { max_bytes } => write!(
                formatter,
                "the authorization server answered with more than {max_bytes} bytes"
            ),
            Self::Transport(source) => write!(
                formatter,
                "the authorization server could not be reached: {source}"
            ),
            Self::UnexpectedAnswer { description } => write!(
                formatter,
                "the authorization server answered unexpectedly: {description}"
            ),
            Self::UnsupportedTokenType { token_type } => write!(
                formatter,
                "the authorization server issued a token of the unsupported type '{}'",
                token_type.as_ref()
            ),
        }
    }
}

impl From<IssuerExchangeError> for ServerUnavailability {
    fn from(failure: IssuerExchangeError) -> Self {
        match failure {
            IssuerExchangeError::Oversized { max_bytes } => Self::Oversized { max_bytes },
            IssuerExchangeError::Transport(source) => Self::Transport(source),
        }
    }
}
