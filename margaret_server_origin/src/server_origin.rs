use std::str::FromStr;

use url::Origin;
use url::Url;

use crate::server_origin_error::ServerOriginError;

#[derive(Clone, Debug)]
pub struct ServerOrigin {
    pub origin: Origin,
}

impl ServerOrigin {
    fn of(url: Url) -> Result<Self, ServerOriginError> {
        let origin = url.origin();

        if origin.is_tuple()
            && url.path() == "/"
            && url.query().is_none()
            && url.fragment().is_none()
            && url.username().is_empty()
            && url.password().is_none()
        {
            Ok(Self { origin })
        } else {
            Err(ServerOriginError::NotAnOrigin { url })
        }
    }
}

impl FromStr for ServerOrigin {
    type Err = ServerOriginError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Url::parse(input)
            .map_err(|source| ServerOriginError::Malformed { source })
            .and_then(Self::of)
    }
}

#[cfg(test)]
mod tests {
    use super::ServerOrigin;

    fn rejection(input: &str) -> String {
        input
            .parse::<ServerOrigin>()
            .expect_err("the server url is rejected")
            .to_string()
    }

    #[test]
    fn serializes_an_origin_without_a_trailing_slash() {
        let server: ServerOrigin = "https://localhost:8443/"
            .parse()
            .expect("the server url is an origin");

        assert_eq!(
            server.origin.ascii_serialization(),
            "https://localhost:8443"
        );
    }

    #[test]
    fn rejects_a_malformed_server_url() {
        assert_eq!(
            rejection("/relative"),
            "the server url is malformed: relative URL without a base"
        );
    }

    #[test]
    fn rejects_a_server_url_with_a_path() {
        assert_eq!(
            rejection("https://localhost/blog"),
            "the server url https://localhost/blog carries more than its scheme, host and port"
        );
    }

    #[test]
    fn rejects_a_server_url_with_a_query() {
        assert_eq!(
            rejection("https://localhost/?tenant=1"),
            "the server url https://localhost/?tenant=1 carries more than its scheme, host and port"
        );
    }

    #[test]
    fn rejects_a_server_url_with_a_fragment() {
        assert_eq!(
            rejection("https://localhost/#top"),
            "the server url https://localhost/#top carries more than its scheme, host and port"
        );
    }

    #[test]
    fn rejects_a_server_url_with_a_user_name() {
        assert_eq!(
            rejection("https://reader@localhost/"),
            "the server url https://reader@localhost/ carries more than its scheme, host and port"
        );
    }

    #[test]
    fn rejects_a_server_url_with_a_password() {
        assert_eq!(
            rejection("https://:secret@localhost/"),
            "the server url https://:secret@localhost/ carries more than its scheme, host and port"
        );
    }

    #[test]
    fn rejects_a_server_url_without_a_tuple_origin() {
        assert_eq!(
            rejection("data:text/plain,margaret"),
            "the server url data:text/plain,margaret carries more than its scheme, host and port"
        );
    }
}
