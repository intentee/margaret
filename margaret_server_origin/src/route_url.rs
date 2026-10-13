use url::Url;

use crate::server_origin_error::ServerOriginError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RouteUrl {
    url: Url,
}

impl RouteUrl {
    /// # Errors
    ///
    /// Returns `ServerOriginError::MalformedRouteUrl` when the url composed for a route is not a
    /// url.
    pub fn parse(url: String) -> Result<Self, ServerOriginError> {
        Url::parse(&url)
            .map(|parsed| Self { url: parsed })
            .map_err(|source| ServerOriginError::MalformedRouteUrl { source, url })
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        self.url.as_str()
    }
}

impl From<RouteUrl> for String {
    fn from(RouteUrl { url }: RouteUrl) -> Self {
        url.into()
    }
}

impl From<RouteUrl> for Url {
    fn from(RouteUrl { url }: RouteUrl) -> Self {
        url
    }
}

#[cfg(test)]
mod tests {
    use url::Url;

    use super::RouteUrl;
    use crate::server_origin_error::ServerOriginError;

    fn callback() -> RouteUrl {
        RouteUrl::parse("https://localhost:8443/oauth/%7Bcallback%7D".to_string())
            .expect("the route url is a url")
    }

    #[test]
    fn keeps_the_encoded_route_url_as_composed() {
        assert_eq!(
            callback().as_str(),
            "https://localhost:8443/oauth/%7Bcallback%7D"
        );
    }

    #[test]
    fn hands_its_text_to_a_consumer_of_strings() {
        assert_eq!(
            String::from(callback()),
            "https://localhost:8443/oauth/%7Bcallback%7D"
        );
    }

    #[test]
    fn hands_its_url_to_a_consumer_of_urls() {
        assert_eq!(Url::from(callback()).path(), "/oauth/%7Bcallback%7D");
    }

    #[test]
    fn rejects_a_route_url_that_is_not_a_url() {
        assert!(matches!(
            RouteUrl::parse("/sign-in/callback".to_string()),
            Err(ServerOriginError::MalformedRouteUrl { url, .. }) if url == "/sign-in/callback"
        ));
    }
}
