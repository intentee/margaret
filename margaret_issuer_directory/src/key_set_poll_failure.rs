use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use reqwest::StatusCode;

use margaret_issuer_request::issuer_exchange_error::IssuerExchangeError;
use margaret_jws_verification::key_set_document_rejection::KeySetDocumentRejection;

use crate::discovery_failure::DiscoveryFailure;

#[derive(Debug)]
pub(crate) enum KeySetPollFailure {
    Discovery(DiscoveryFailure),
    DocumentExchange(IssuerExchangeError),
    DocumentOversized { max_bytes: usize },
    DocumentRejected(KeySetDocumentRejection),
    DocumentStatus(StatusCode),
}

impl Display for KeySetPollFailure {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::Discovery(failure) => {
                write!(formatter, "the key set could not be located: {failure}")
            }
            Self::DocumentExchange(failure) => write!(
                formatter,
                "the key set document could not be fetched: {failure}"
            ),
            Self::DocumentOversized { max_bytes } => {
                write!(formatter, "the key set document exceeds {max_bytes} bytes")
            }
            Self::DocumentRejected(rejection) => {
                write!(formatter, "the key set document is rejected: {rejection}")
            }
            Self::DocumentStatus(status) => write!(
                formatter,
                "the issuer answered the key set request with status {status}"
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use reqwest::StatusCode;

    use margaret_issuer_request::issuer_exchange_error::IssuerExchangeError;
    use margaret_jws_verification::key_set_document_rejection::KeySetDocumentRejection;

    use super::KeySetPollFailure;
    use crate::discovery_failure::DiscoveryFailure;

    #[test]
    fn describes_every_failure() {
        let described = [
            KeySetPollFailure::Discovery(DiscoveryFailure::Status(StatusCode::NOT_FOUND)),
            KeySetPollFailure::DocumentExchange(IssuerExchangeError::Transport(
                reqwest::Client::new()
                    .get("not a url")
                    .build()
                    .expect_err("a relative url is not requestable"),
            )),
            KeySetPollFailure::DocumentOversized { max_bytes: 16 },
            KeySetPollFailure::DocumentRejected(KeySetDocumentRejection::Malformed {
                source: serde_json::from_str::<u8>("x").expect_err("not json"),
            }),
            KeySetPollFailure::DocumentStatus(StatusCode::SERVICE_UNAVAILABLE),
        ]
        .map(|failure| failure.to_string());

        assert_eq!(
            described[0],
            "the key set could not be located: the issuer answered the discovery request with status 404 Not Found"
        );
        assert!(described[1].starts_with(
            "the key set document could not be fetched: the issuer could not be reached: "
        ));
        assert_eq!(described[2], "the key set document exceeds 16 bytes");
        assert!(described[3].starts_with("the key set document is rejected: "));
        assert_eq!(
            described[4],
            "the issuer answered the key set request with status 503 Service Unavailable"
        );
    }
}
