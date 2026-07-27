use std::str::FromStr;
use std::sync::Arc;

use url::Url;

use crate::route_origin_error::RouteOriginError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RouteOrigin {
    serialized: Arc<str>,
}

impl RouteOrigin {
    pub fn parse(value: &str) -> Result<Self, RouteOriginError> {
        let parsed = Url::parse(value)?;
        let serialized = parsed.origin().ascii_serialization();
        let canonical = parsed.scheme() == "https"
            && parsed.host().is_some()
            && parsed.username().is_empty()
            && parsed.password().is_none()
            && parsed.path() == "/"
            && parsed.query().is_none()
            && parsed.fragment().is_none()
            && value == serialized;

        if !canonical {
            return Err(RouteOriginError::NonCanonicalHttpsOrigin);
        }

        Ok(Self {
            serialized: serialized.into(),
        })
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.serialized
    }
}

impl FromStr for RouteOrigin {
    type Err = RouteOriginError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

#[cfg(test)]
mod tests {
    use super::RouteOrigin;

    #[test]
    fn accepts_a_canonical_https_origin() {
        let origin = RouteOrigin::parse("https://example.test:8443")
            .expect("a canonical HTTPS origin is accepted");

        assert_eq!(origin.as_str(), "https://example.test:8443");
    }

    #[test]
    fn rejects_an_invalid_url() {
        assert!(RouteOrigin::parse("not a url").is_err());
    }

    #[test]
    fn parses_through_the_standard_typed_input_contract() {
        assert!("https://example.test".parse::<RouteOrigin>().is_ok());
        assert!("http://example.test".parse::<RouteOrigin>().is_err());
    }

    #[test]
    fn rejects_non_https_and_non_origin_urls() {
        for value in [
            "http://example.test",
            "https://user@example.test",
            "https://example.test/",
            "https://example.test/path",
            "https://example.test?query",
            "https://example.test#fragment",
            "https://example.test:443",
        ] {
            assert!(RouteOrigin::parse(value).is_err(), "{value}");
        }
    }
}
