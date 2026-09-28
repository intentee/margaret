use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use reqwest::StatusCode;

use margaret_jws_verification::key_set_rejection::KeySetRejection;

#[derive(Debug)]
pub enum KeySetPollFailure<TLocationFailure> {
    DocumentRejected(KeySetRejection),
    DocumentStatus(StatusCode),
    DocumentTransport(reqwest::Error),
    Location(TLocationFailure),
}

impl<TLocationFailure: Display> Display for KeySetPollFailure<TLocationFailure> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
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

    use margaret_jws_verification::key_set_rejection::KeySetRejection;

    use super::KeySetPollFailure;

    #[test]
    fn describes_every_failure() {
        let described = [
            KeySetPollFailure::DocumentRejected(KeySetRejection::Malformed {
                source: serde_json::from_str::<u8>("x").expect_err("not json"),
            }),
            KeySetPollFailure::DocumentStatus(StatusCode::SERVICE_UNAVAILABLE),
            KeySetPollFailure::DocumentTransport(
                Client::new()
                    .get("https://")
                    .build()
                    .expect_err("an empty host is not a request"),
            ),
            KeySetPollFailure::Location("the fixture locator failed"),
        ]
        .map(|failure| failure.to_string());

        assert!(described[0].starts_with("the key set document is rejected: "));
        assert_eq!(
            described[1],
            "the issuer answered the key set request with status 503 Service Unavailable"
        );
        assert!(
            described[2]
                .starts_with("the key set document could not be transferred from the issuer: ")
        );
        assert_eq!(
            described[3],
            "the key set could not be located: the fixture locator failed"
        );
    }
}
