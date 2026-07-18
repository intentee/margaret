use http::HeaderName;
use http::header::SET_COOKIE;

use crate::response_header_name_error::ResponseHeaderNameError;

pub struct ResponseHeaderName {
    header_name: HeaderName,
}

impl ResponseHeaderName {
    pub fn new(name: &str) -> Result<Self, ResponseHeaderNameError> {
        let header_name = HeaderName::from_bytes(name.as_bytes()).map_err(|source| {
            ResponseHeaderNameError::MalformedName {
                name: name.to_owned(),
                source,
            }
        })?;

        if header_name == SET_COOKIE {
            return Err(ResponseHeaderNameError::CookieJarOwnedName);
        }

        Ok(Self { header_name })
    }

    #[must_use]
    pub fn into_header_name(self) -> HeaderName {
        self.header_name
    }
}

#[cfg(test)]
mod tests {
    use http::HeaderName;

    use crate::response_header_name::ResponseHeaderName;
    use crate::response_header_name_error::ResponseHeaderNameError;

    #[test]
    fn accepts_a_custom_response_header_name() {
        let response_header_name =
            ResponseHeaderName::new("x-marker").expect("`x-marker` must be an accepted name");

        assert_eq!(
            response_header_name.into_header_name(),
            HeaderName::from_static("x-marker")
        );
    }

    #[test]
    fn normalizes_a_header_name_written_in_upper_case() {
        let response_header_name =
            ResponseHeaderName::new("X-Marker").expect("`X-Marker` must be an accepted name");

        assert_eq!(
            response_header_name.into_header_name(),
            HeaderName::from_static("x-marker")
        );
    }

    #[test]
    fn rejects_the_set_cookie_header_name() {
        let error = ResponseHeaderName::new("set-cookie")
            .err()
            .expect("`set-cookie` must be rejected");

        assert_eq!(
            error.to_string(),
            "the `set-cookie` response header is produced by the cookie jar and cannot be set directly"
        );
    }

    #[test]
    fn rejects_the_set_cookie_header_name_written_in_upper_case() {
        let error = ResponseHeaderName::new("Set-Cookie")
            .err()
            .expect("`Set-Cookie` must be rejected");

        assert_eq!(
            error.to_string(),
            "the `set-cookie` response header is produced by the cookie jar and cannot be set directly"
        );
    }

    #[test]
    fn rejects_a_malformed_header_name() {
        let error = ResponseHeaderName::new("x marker")
            .err()
            .expect("`x marker` must be rejected");

        assert!(matches!(
            error,
            ResponseHeaderNameError::MalformedName { ref name, .. } if name == "x marker"
        ));
    }
}
