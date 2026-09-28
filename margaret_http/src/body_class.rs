use http::header::CONTENT_TYPE;
use mime::Mime;

use crate::request_outcome::RequestOutcome;
use crate::request_rejection::RequestRejection;
use crate::server_params::ServerParams;

fn is_multipart(media: &Mime) -> bool {
    media.type_() == mime::MULTIPART && media.subtype() == mime::FORM_DATA
}

fn is_urlencoded(media: &Mime) -> bool {
    media.type_() == mime::APPLICATION && media.subtype() == mime::WWW_FORM_URLENCODED
}

fn is_json(media: &Mime) -> bool {
    media.type_() == mime::APPLICATION && media.subtype() == mime::JSON
}

pub(crate) enum BodyClass {
    Multipart { boundary: String },
    UrlEncoded,
    Json,
    Other,
}

impl BodyClass {
    pub(crate) fn from_server_params(server: &ServerParams) -> RequestOutcome<Self> {
        let Some(value) = server.header(&CONTENT_TYPE) else {
            return RequestOutcome::Parsed(Self::Other);
        };

        let media = match value.parse::<Mime>() {
            Ok(media) => media,
            Err(source) => {
                return RequestOutcome::Rejected(RequestRejection::MalformedContentType { source });
            }
        };

        if is_multipart(&media) {
            match media.get_param(mime::BOUNDARY) {
                Some(boundary) => RequestOutcome::Parsed(Self::Multipart {
                    boundary: boundary.as_str().to_string(),
                }),
                None => RequestOutcome::Rejected(RequestRejection::MissingMultipartBoundary),
            }
        } else if is_urlencoded(&media) {
            RequestOutcome::Parsed(Self::UrlEncoded)
        } else if is_json(&media) {
            RequestOutcome::Parsed(Self::Json)
        } else {
            RequestOutcome::Parsed(Self::Other)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem::discriminant;
    use std::net::SocketAddr;

    use http::HeaderValue;
    use http::Method;
    use http::Request;
    use http::header::CONTENT_TYPE;
    use http::header::HOST;
    use mime::Mime;

    use super::BodyClass;
    use crate::request_outcome::RequestOutcome;
    use crate::request_rejection::RequestRejection;
    use crate::server_params::ServerParams;

    fn outcome(content_type: &[u8]) -> Result<BodyClass, RequestRejection> {
        let parts = Request::builder()
            .method(Method::POST)
            .uri("/")
            .header(HOST, "localhost")
            .header(
                CONTENT_TYPE,
                HeaderValue::from_bytes(content_type).expect("a header value"),
            )
            .body(())
            .expect("a request")
            .into_parts()
            .0;

        match ServerParams::from_parts(parts, SocketAddr::from(([127, 0, 0, 1], 0))) {
            RequestOutcome::Parsed(server) => match BodyClass::from_server_params(&server) {
                RequestOutcome::Parsed(body_class) => Ok(body_class),
                RequestOutcome::Rejected(rejection) => Err(rejection),
            },
            RequestOutcome::Rejected(rejection) => Err(rejection),
        }
    }

    fn assert_classifies(content_type: &[u8], expected: &BodyClass) {
        assert_eq!(
            discriminant(&outcome(content_type).expect("the content type is unambiguous")),
            discriminant(expected)
        );
    }

    fn assert_rejects(content_type: &[u8], expected: &RequestRejection) {
        assert_eq!(
            discriminant(
                &outcome(content_type)
                    .err()
                    .expect("the content type is ambiguous")
            ),
            discriminant(expected)
        );
    }

    #[test]
    fn reports_a_content_type_that_is_not_a_media_type() {
        assert_rejects(
            b"not/a/media/type",
            &RequestRejection::MalformedContentType {
                source: "//".parse::<Mime>().expect_err("a malformed mime"),
            },
        );
    }

    #[test]
    fn reports_a_content_type_that_is_not_visible_ascii() {
        assert_rejects(
            b"text/plain; charset=\xff",
            &RequestRejection::HeaderValueNotVisibleAscii {
                name: CONTENT_TYPE,
                source: HeaderValue::from_bytes(&[0xC0])
                    .expect("a raw header value")
                    .to_str()
                    .expect_err("raw bytes are not visible ASCII"),
            },
        );
    }

    #[test]
    fn reports_a_multipart_body_without_a_boundary() {
        assert_rejects(
            b"multipart/form-data",
            &RequestRejection::MissingMultipartBoundary,
        );
    }

    #[test]
    fn classifies_a_multipart_body_with_its_boundary() {
        assert_classifies(
            b"multipart/form-data; boundary=X",
            &BodyClass::Multipart {
                boundary: String::new(),
            },
        );
    }

    #[test]
    fn classifies_an_unrecognized_media_type_as_other() {
        assert_classifies(b"text/plain", &BodyClass::Other);
    }
}
