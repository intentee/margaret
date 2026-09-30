use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::discovery_failure::DiscoveryFailure;

#[derive(Debug)]
pub(crate) enum KeySetLocationFailure {
    Discovery(DiscoveryFailure),
    Endpoint(anyhow::Error),
}

impl Display for KeySetLocationFailure {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::Discovery(failure) => failure.fmt(formatter),
            Self::Endpoint(source) => write!(
                formatter,
                "the key set endpoint could not be provided: {source}"
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use reqwest::StatusCode;

    use super::KeySetLocationFailure;
    use crate::discovery_failure::DiscoveryFailure;

    #[test]
    fn describes_a_discovery_failure_as_the_discovery_failure() {
        assert_eq!(
            KeySetLocationFailure::Discovery(DiscoveryFailure::Status(StatusCode::NOT_FOUND))
                .to_string(),
            DiscoveryFailure::Status(StatusCode::NOT_FOUND).to_string()
        );
    }

    #[test]
    fn describes_an_endpoint_that_cannot_be_provided() {
        assert_eq!(
            KeySetLocationFailure::Endpoint(anyhow::anyhow!("the endpoint is unresolvable"))
                .to_string(),
            "the key set endpoint could not be provided: the endpoint is unresolvable"
        );
    }
}
