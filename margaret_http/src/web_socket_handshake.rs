use base64ct::Base64;
use base64ct::Encoding;
use http::HeaderMap;
use http::HeaderValue;
use http::header::CONNECTION;
use http::header::CONTENT_LENGTH;
use http::header::ORIGIN;
use http::header::TRANSFER_ENCODING;
use http::header::UPGRADE;

use crate::response::Response;

fn exactly_one<'headers>(
    headers: &'headers HeaderMap,
    name: &http::HeaderName,
) -> Option<&'headers HeaderValue> {
    let mut values = headers.get_all(name).iter();
    let value = values.next()?;

    values.next().is_none().then_some(value)
}

fn contains_upgrade_token(value: &HeaderValue) -> bool {
    value
        .as_bytes()
        .split(|byte| *byte == b',')
        .map(|token| token.trim_ascii())
        .any(|token| token.eq_ignore_ascii_case(b"upgrade"))
}

fn valid_key(value: &HeaderValue) -> bool {
    value
        .to_str()
        .ok()
        .and_then(|key| Base64::decode_vec(key).ok())
        .is_some_and(|decoded| decoded.len() == 16)
}

pub(crate) fn web_socket_handshake_rejection(
    headers: &HeaderMap,
    expected_origin: &HeaderValue,
) -> Option<Response> {
    let valid = exactly_one(headers, &CONNECTION).is_some_and(contains_upgrade_token)
        && exactly_one(headers, &UPGRADE)
            .is_some_and(|value| value.as_bytes().eq_ignore_ascii_case(b"websocket"))
        && exactly_one(headers, &ORIGIN).is_some_and(|value| value == expected_origin)
        && exactly_one(
            headers,
            &http::header::HeaderName::from_static("sec-websocket-version"),
        )
        .is_some_and(|value| value == "13")
        && exactly_one(
            headers,
            &http::header::HeaderName::from_static("sec-websocket-key"),
        )
        .is_some_and(valid_key)
        && !headers.contains_key(CONTENT_LENGTH)
        && !headers.contains_key(TRANSFER_ENCODING)
        && !headers.contains_key("sec-websocket-extensions")
        && !headers.contains_key("sec-websocket-protocol");

    (!valid).then(|| Response::text(400, "Bad Request"))
}

#[cfg(test)]
mod tests {
    use http::HeaderMap;
    use http::HeaderValue;

    use super::web_socket_handshake_rejection;

    fn valid_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            "connection",
            HeaderValue::from_static("keep-alive, Upgrade"),
        );
        headers.insert("upgrade", HeaderValue::from_static("websocket"));
        headers.insert("origin", HeaderValue::from_static("https://example.test"));
        headers.insert("sec-websocket-version", HeaderValue::from_static("13"));
        headers.insert(
            "sec-websocket-key",
            HeaderValue::from_static("dGhlIHNhbXBsZSBub25jZQ=="),
        );
        headers
    }

    fn rejected(headers: &HeaderMap) -> bool {
        web_socket_handshake_rejection(headers, &HeaderValue::from_static("https://example.test"))
            .is_some()
    }

    #[test]
    fn accepts_a_complete_exact_handshake() {
        assert!(!rejected(&valid_headers()));
    }

    #[test]
    fn rejects_missing_or_duplicate_required_headers() {
        let mut missing = valid_headers();
        missing.remove("origin");
        assert!(rejected(&missing));

        let mut duplicate = valid_headers();
        duplicate.append("origin", HeaderValue::from_static("https://example.test"));
        assert!(rejected(&duplicate));
    }

    #[test]
    fn rejects_an_unexpected_origin() {
        let mut headers = valid_headers();
        headers.insert("origin", HeaderValue::from_static("https://evil.test"));

        assert!(rejected(&headers));
    }

    #[test]
    fn rejects_invalid_upgrade_headers() {
        for (name, value) in [
            ("connection", "close"),
            ("upgrade", "h2c"),
            ("sec-websocket-version", "12"),
            ("sec-websocket-key", "not-base64"),
        ] {
            let mut headers = valid_headers();
            headers.insert(
                http::HeaderName::try_from(name).expect("the test header name is valid"),
                HeaderValue::try_from(value).expect("the test header value is valid"),
            );

            assert!(rejected(&headers));
        }
    }

    #[test]
    fn rejects_bodies_extensions_and_unnegotiated_protocols() {
        for (name, value) in [
            ("content-length", "0"),
            ("transfer-encoding", "chunked"),
            ("sec-websocket-extensions", "permessage-deflate"),
            ("sec-websocket-protocol", "chat"),
        ] {
            let mut headers = valid_headers();
            headers.insert(
                http::HeaderName::try_from(name).expect("the test header name is valid"),
                HeaderValue::try_from(value).expect("the test header value is valid"),
            );

            assert!(rejected(&headers));
        }
    }
}
