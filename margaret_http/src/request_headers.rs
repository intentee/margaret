use std::collections::HashMap;

use http::HeaderMap;
use http::HeaderName;

use crate::request_outcome::RequestOutcome;
use crate::request_rejection::RequestRejection;
use crate::singleton_request_header::SingletonRequestHeader;

const COMBINED_FIELD_VALUE_SEPARATOR: &str = ", ";

pub(crate) struct RequestHeaders {
    values: HashMap<HeaderName, String>,
}

impl RequestHeaders {
    pub(crate) fn from_header_map(headers: &HeaderMap) -> RequestOutcome<Self> {
        let mut values = HashMap::with_capacity(headers.keys_len());

        for name in headers.keys() {
            let mut field_lines = Vec::new();

            for value in headers.get_all(name) {
                match value.to_str() {
                    Ok(text) => field_lines.push(text),
                    Err(source) => {
                        return RequestOutcome::Rejected(
                            RequestRejection::HeaderValueNotVisibleAscii {
                                name: name.clone(),
                                source,
                            },
                        );
                    }
                }
            }

            if field_lines.len() > 1
                && let Some(header) = SingletonRequestHeader::classify(name)
            {
                return RequestOutcome::Rejected(RequestRejection::RepeatedSingletonHeader {
                    header,
                });
            }

            values.insert(
                name.clone(),
                field_lines.join(COMBINED_FIELD_VALUE_SEPARATOR),
            );
        }

        RequestOutcome::Parsed(Self { values })
    }

    pub(crate) fn get(&self, name: &HeaderName) -> Option<&str> {
        self.values.get(name).map(String::as_str)
    }
}

#[cfg(test)]
mod tests {
    use std::mem::discriminant;

    use http::HeaderMap;
    use http::HeaderName;
    use http::HeaderValue;
    use http::header::ACCEPT_ENCODING;
    use http::header::COOKIE;

    use super::RequestHeaders;
    use crate::request_outcome::RequestOutcome;
    use crate::request_rejection::RequestRejection;
    use crate::singleton_request_header::SingletonRequestHeader;

    fn outcome(lines: &[(HeaderName, &[u8])]) -> Result<RequestHeaders, RequestRejection> {
        let mut headers = HeaderMap::new();

        for (name, value) in lines {
            headers.append(
                name.clone(),
                HeaderValue::from_bytes(value).expect("a header value"),
            );
        }

        match RequestHeaders::from_header_map(&headers) {
            RequestOutcome::Parsed(headers) => Ok(headers),
            RequestOutcome::Rejected(rejection) => Err(rejection),
        }
    }

    fn parsed(lines: &[(HeaderName, &[u8])]) -> RequestHeaders {
        outcome(lines).expect("the header set is unambiguous")
    }

    fn assert_rejects(lines: &[(HeaderName, &[u8])], expected: &RequestRejection) {
        assert_eq!(
            discriminant(&outcome(lines).err().expect("the header set is ambiguous")),
            discriminant(expected)
        );
    }

    #[test]
    fn reads_a_single_field_line() {
        assert_eq!(
            parsed(&[(COOKIE, b"session=abc")]).get(&COOKIE),
            Some("session=abc")
        );
    }

    #[test]
    fn reports_no_value_for_an_absent_header() {
        assert_eq!(parsed(&[]).get(&COOKIE), None);
    }

    #[test]
    fn rejects_a_repeated_singleton_header() {
        assert_rejects(
            &[(COOKIE, b"session=abc"), (COOKIE, b"session=stolen")],
            &RequestRejection::RepeatedSingletonHeader {
                header: SingletonRequestHeader::Cookie,
            },
        );
    }

    #[test]
    fn combines_a_repeated_list_valued_header() {
        assert_eq!(
            parsed(&[(ACCEPT_ENCODING, b"gzip"), (ACCEPT_ENCODING, b"br")]).get(&ACCEPT_ENCODING),
            Some("gzip, br")
        );
    }

    #[test]
    fn rejects_a_header_value_that_is_not_visible_ascii() {
        assert_rejects(
            &[(COOKIE, &[0xC0, 0xC1])],
            &RequestRejection::HeaderValueNotVisibleAscii {
                name: COOKIE,
                source: HeaderValue::from_bytes(&[0xC0])
                    .expect("a raw header value")
                    .to_str()
                    .expect_err("raw bytes are not visible ASCII"),
            },
        );
    }
}
