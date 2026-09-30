use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use reqwest::StatusCode;

use margaret_jws_verification::key_set_document_rejection::KeySetDocumentRejection;

use crate::key_set_location_failure::KeySetLocationFailure;

#[derive(Debug)]
pub(crate) enum KeySetPollFailure {
    DocumentOversized { max_bytes: usize },
    DocumentRejected(KeySetDocumentRejection),
    DocumentStatus(StatusCode),
    DocumentTransport(reqwest::Error),
    Location(KeySetLocationFailure),
}

impl Display for KeySetPollFailure {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::DocumentOversized { max_bytes } => write!(
                formatter,
                "the key set document is larger than {max_bytes} bytes"
            ),
            Self::DocumentRejected(rejection) => {
                write!(formatter, "the key set document is rejected: {rejection}")
            }
            Self::DocumentStatus(status) => write!(
                formatter,
                "the issuer answered the key set request with status {status}"
            ),
            Self::DocumentTransport(source) => write!(
                formatter,
                "the key set document could not be transferred from the issuer: {source}"
            ),
            Self::Location(failure) => {
                write!(formatter, "the key set could not be located: {failure}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use reqwest::Client;
    use reqwest::StatusCode;

    use margaret_jws_verification::key_set_document_rejection::KeySetDocumentRejection;

    use super::KeySetPollFailure;
    use crate::key_set_location_failure::KeySetLocationFailure;

    #[test]
    fn describes_every_failure() {
        let described = [
            KeySetPollFailure::DocumentOversized { max_bytes: 16 },
            KeySetPollFailure::DocumentRejected(KeySetDocumentRejection::Malformed {
                source: serde_json::from_str::<u8>("x").expect_err("not json"),
            }),
            KeySetPollFailure::DocumentStatus(StatusCode::SERVICE_UNAVAILABLE),
            KeySetPollFailure::DocumentTransport(
                Client::new()
                    .get("https://")
                    .build()
                    .expect_err("an empty host is not a request"),
            ),
            KeySetPollFailure::Location(KeySetLocationFailure::Endpoint(anyhow::anyhow!(
                "the endpoint is unresolvable"
            ))),
        ]
        .map(|failure| failure.to_string());

        assert_eq!(described[0], "the key set document is larger than 16 bytes");
        assert!(described[1].starts_with("the key set document is rejected: "));
        assert_eq!(
            described[2],
            "the issuer answered the key set request with status 503 Service Unavailable"
        );
        assert!(
            described[3]
                .starts_with("the key set document could not be transferred from the issuer: ")
        );
        assert_eq!(
            described[4],
            "the key set could not be located: the key set endpoint could not be provided: the endpoint is unresolvable"
        );
    }
}
