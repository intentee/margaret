use url::Url;

use crate::https_url_parsing::HttpsUrlParsing;
use crate::https_url_rejection::HttpsUrlRejection;

const HTTPS_SCHEME: &str = "https";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpsUrl {
    url: Url,
}

impl HttpsUrl {
    #[must_use]
    pub fn parse(original: &str) -> HttpsUrlParsing {
        match Url::parse(original) {
            Ok(url) if url.scheme() == HTTPS_SCHEME => HttpsUrlParsing::Accepted(Self { url }),
            Ok(url) => HttpsUrlParsing::Rejected(HttpsUrlRejection::NotHttps {
                scheme: url.scheme().to_string(),
            }),
            Err(source) => HttpsUrlParsing::Rejected(HttpsUrlRejection::Malformed(source)),
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        self.url.as_str()
    }

    #[must_use]
    pub fn url(&self) -> &Url {
        &self.url
    }
}

impl From<HttpsUrl> for Url {
    fn from(HttpsUrl { url }: HttpsUrl) -> Self {
        url
    }
}

#[cfg(test)]
mod tests {
    use url::Url;

    use super::HttpsUrl;
    use crate::https_url_parsing::HttpsUrlParsing;
    use crate::https_url_rejection::HttpsUrlRejection;

    #[test]
    fn accepts_an_https_url_in_its_canonical_form() {
        assert!(matches!(
            HttpsUrl::parse("https://issuer.example"),
            HttpsUrlParsing::Accepted(url) if url.as_str() == "https://issuer.example/"
        ));
    }

    #[test]
    fn converts_into_the_url_it_validated() {
        assert!(matches!(
            HttpsUrl::parse("https://issuer.example/jwks.json"),
            HttpsUrlParsing::Accepted(url) if Url::from(url.clone()) == *url.url()
        ));
    }

    #[test]
    fn rejects_a_plaintext_url() {
        assert_eq!(
            HttpsUrl::parse("http://issuer.example"),
            HttpsUrlParsing::Rejected(HttpsUrlRejection::NotHttps {
                scheme: "http".to_string()
            })
        );
    }

    #[test]
    fn rejects_a_relative_reference() {
        assert_eq!(
            HttpsUrl::parse("/jwks.json"),
            HttpsUrlParsing::Rejected(HttpsUrlRejection::Malformed(
                url::ParseError::RelativeUrlWithoutBase
            ))
        );
    }

    #[test]
    fn describes_every_rejection() {
        assert_eq!(
            [
                HttpsUrlRejection::Malformed(url::ParseError::RelativeUrlWithoutBase),
                HttpsUrlRejection::NotHttps {
                    scheme: "http".to_string(),
                },
            ]
            .map(|rejection| rejection.to_string()),
            [
                "the url is malformed: relative URL without a base",
                "the url uses the 'http' scheme instead of https",
            ]
        );
    }
}
