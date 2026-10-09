use std::str::FromStr;

use url::Url;

use margaret_https_url::https_url::HttpsUrl;
use margaret_https_url::https_url_parsing::HttpsUrlParsing;

use crate::sessions_error::SessionsError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionRefreshUrl {
    url: HttpsUrl,
}

impl SessionRefreshUrl {
    #[must_use]
    pub fn url(&self) -> &Url {
        self.url.url()
    }
}

impl FromStr for SessionRefreshUrl {
    type Err = SessionsError;

    fn from_str(url: &str) -> Result<Self, Self::Err> {
        match HttpsUrl::parse(url) {
            HttpsUrlParsing::Accepted(url) => Ok(Self { url }),
            HttpsUrlParsing::Rejected(rejection) => {
                Err(SessionsError::MalformedRefreshUrl(rejection))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use margaret_https_url::https_url_rejection::HttpsUrlRejection;

    use super::SessionRefreshUrl;
    use crate::sessions_error::SessionsError;

    #[test]
    fn reads_an_https_url() {
        assert_eq!(
            "https://identity.internal/sessions/refresh"
                .parse::<SessionRefreshUrl>()
                .expect("the url is read")
                .url()
                .as_str(),
            "https://identity.internal/sessions/refresh"
        );
    }

    #[test]
    fn rejects_a_url_of_another_scheme() {
        assert!(matches!(
            "http://identity.internal/sessions/refresh".parse::<SessionRefreshUrl>(),
            Err(SessionsError::MalformedRefreshUrl(HttpsUrlRejection::NotHttps { scheme })) if scheme == "http"
        ));
    }
}
