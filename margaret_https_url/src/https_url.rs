use std::str::FromStr;

use url::Url;

use crate::https_url_error::HttpsUrlError;

const HTTPS_SCHEME: &str = "https";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpsUrl {
    url: Url,
}

impl HttpsUrl {
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

impl FromStr for HttpsUrl {
    type Err = HttpsUrlError;

    fn from_str(original: &str) -> Result<Self, Self::Err> {
        let url = Url::parse(original).map_err(|source| HttpsUrlError::Malformed { source })?;

        if url.scheme() == HTTPS_SCHEME {
            Ok(Self { url })
        } else {
            Err(HttpsUrlError::NotHttps {
                scheme: url.scheme().to_string(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use url::Url;

    use super::HttpsUrl;
    use crate::https_url_error::HttpsUrlError;

    #[test]
    fn accepts_an_https_url_in_its_canonical_form() {
        let url: HttpsUrl = "https://issuer.example"
            .parse()
            .expect("the url uses https");

        assert_eq!(url.as_str(), "https://issuer.example/");
    }

    #[test]
    fn converts_into_the_url_it_validated() {
        let url: HttpsUrl = "https://issuer.example/jwks.json"
            .parse()
            .expect("the url uses https");

        assert_eq!(Url::from(url.clone()), *url.url());
    }

    #[test]
    fn rejects_a_plaintext_url() {
        assert_eq!(
            "http://issuer.example"
                .parse::<HttpsUrl>()
                .expect_err("the url does not use https")
                .to_string(),
            "the url uses the 'http' scheme instead of https"
        );
    }

    #[test]
    fn rejects_a_relative_reference() {
        let error = "/jwks.json"
            .parse::<HttpsUrl>()
            .expect_err("a relative reference is not a url");

        assert_eq!(
            error.to_string(),
            "the url is malformed: relative URL without a base"
        );
        assert!(matches!(
            error,
            HttpsUrlError::Malformed { source } if source == url::ParseError::RelativeUrlWithoutBase
        ));
    }
}
