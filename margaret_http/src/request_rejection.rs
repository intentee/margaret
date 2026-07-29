use std::fmt::Display;
use std::fmt::Formatter;
use std::str::Utf8Error;

use cookie::ParseError;
use http::HeaderName;
use http::header::ToStrError;
use http::uri::InvalidUri;

use crate::response::Response;
use crate::singleton_request_header::SingletonRequestHeader;

#[derive(Debug)]
pub(crate) enum RequestRejection {
    AuthorityMismatch,
    ControlCharacterInPathSegment {
        segment: String,
    },
    DotSegmentInPath {
        segment: String,
    },
    DuplicateCookie {
        name: String,
    },
    DuplicateFormField {
        name: String,
    },
    DuplicateQueryParameter {
        name: String,
    },
    DuplicateUploadField {
        name: String,
    },
    EmptyPathSegment,
    EncodedPathSeparator {
        segment: String,
    },
    HeaderValueNotVisibleAscii {
        name: HeaderName,
        source: ToStrError,
    },
    MalformedContentType {
        source: mime::FromStrError,
    },
    MalformedCookie {
        source: ParseError,
    },
    MalformedHost {
        source: InvalidUri,
    },
    MalformedJson {
        source: serde_json::Error,
    },
    MalformedMultipart {
        source: multer::Error,
    },
    MalformedPercentEncoding {
        segment: String,
    },
    MissingHost,
    MissingMultipartBoundary,
    NamelessMultipartField,
    PathSegmentNotUtf8 {
        segment: String,
        source: Utf8Error,
    },
    PayloadTooLarge {
        limit: u64,
    },
    RepeatedSingletonHeader {
        header: SingletonRequestHeader,
    },
    RequestTargetNotOriginForm,
    UnreadableBody {
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    UploadsDisabled,
}

impl RequestRejection {
    pub(crate) fn into_response(self) -> Response {
        match self {
            Self::PayloadTooLarge { .. } => Response::text(413, "Payload Too Large"),
            Self::AuthorityMismatch
            | Self::ControlCharacterInPathSegment { .. }
            | Self::DotSegmentInPath { .. }
            | Self::DuplicateCookie { .. }
            | Self::DuplicateFormField { .. }
            | Self::DuplicateQueryParameter { .. }
            | Self::DuplicateUploadField { .. }
            | Self::EmptyPathSegment
            | Self::EncodedPathSeparator { .. }
            | Self::HeaderValueNotVisibleAscii { .. }
            | Self::MalformedContentType { .. }
            | Self::MalformedCookie { .. }
            | Self::MalformedHost { .. }
            | Self::MalformedJson { .. }
            | Self::MalformedMultipart { .. }
            | Self::MalformedPercentEncoding { .. }
            | Self::MissingHost
            | Self::MissingMultipartBoundary
            | Self::NamelessMultipartField
            | Self::PathSegmentNotUtf8 { .. }
            | Self::RepeatedSingletonHeader { .. }
            | Self::RequestTargetNotOriginForm
            | Self::UnreadableBody { .. }
            | Self::UploadsDisabled => Response::text(400, "Bad Request"),
        }
    }
}

impl Display for RequestRejection {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AuthorityMismatch => {
                formatter.write_str("the request target authority and the Host header disagree")
            }
            Self::ControlCharacterInPathSegment { segment } => write!(
                formatter,
                "the path segment '{segment}' decodes to a control character"
            ),
            Self::DotSegmentInPath { segment } => {
                write!(
                    formatter,
                    "the request path contains the dot segment '{segment}'"
                )
            }
            Self::DuplicateCookie { name } => {
                write!(formatter, "the request repeats the cookie '{name}'")
            }
            Self::DuplicateFormField { name } => {
                write!(
                    formatter,
                    "the request body repeats the form field '{name}'"
                )
            }
            Self::DuplicateQueryParameter { name } => {
                write!(formatter, "the query string repeats the parameter '{name}'")
            }
            Self::DuplicateUploadField { name } => write!(
                formatter,
                "the request body repeats the upload field '{name}'"
            ),
            Self::EmptyPathSegment => {
                formatter.write_str("the request path contains an empty segment")
            }
            Self::EncodedPathSeparator { segment } => write!(
                formatter,
                "the path segment '{segment}' decodes to a path separator"
            ),
            Self::HeaderValueNotVisibleAscii { name, source } => write!(
                formatter,
                "the '{name}' header value is not visible ASCII text: {source}"
            ),
            Self::MalformedContentType { source } => write!(
                formatter,
                "the request content type is not a valid media type: {source}"
            ),
            Self::MalformedCookie { source } => {
                write!(formatter, "a request cookie could not be parsed: {source}")
            }
            Self::MalformedHost { source } => write!(
                formatter,
                "the Host header is not a valid authority: {source}"
            ),
            Self::MalformedJson { source } => {
                write!(formatter, "the request body is not valid JSON: {source}")
            }
            Self::MalformedMultipart { source } => write!(
                formatter,
                "the multipart request body could not be parsed: {source}"
            ),
            Self::MalformedPercentEncoding { segment } => write!(
                formatter,
                "the path segment '{segment}' is not validly percent encoded"
            ),
            Self::MissingHost => formatter.write_str("the request has no Host header"),
            Self::MissingMultipartBoundary => {
                formatter.write_str("the multipart request is missing its Content-Type boundary")
            }
            Self::NamelessMultipartField => formatter
                .write_str("a multipart request part is missing its Content-Disposition name"),
            Self::PathSegmentNotUtf8 { segment, source } => write!(
                formatter,
                "the path segment '{segment}' does not decode to UTF-8: {source}"
            ),
            Self::PayloadTooLarge { limit } => write!(
                formatter,
                "the request body exceeds the {limit} byte upload limit"
            ),
            Self::RepeatedSingletonHeader { header } => write!(
                formatter,
                "the request repeats the single valued header {header:?}"
            ),
            Self::RequestTargetNotOriginForm => {
                formatter.write_str("the request target is not in origin form")
            }
            Self::UnreadableBody { source } => {
                write!(formatter, "the request body could not be read: {source}")
            }
            Self::UploadsDisabled => formatter
                .write_str("the server received an uploaded file but file uploads are disabled"),
        }
    }
}

#[cfg(test)]
mod tests {
    use http::header::COOKIE;
    use http::header::HeaderValue;
    use http::uri::Authority;
    use percent_encoding::percent_decode_str;

    use super::RequestRejection;
    use crate::singleton_request_header::SingletonRequestHeader;

    fn every_rejection() -> Vec<RequestRejection> {
        vec![
            RequestRejection::AuthorityMismatch,
            RequestRejection::ControlCharacterInPathSegment {
                segment: "a%00b".to_string(),
            },
            RequestRejection::DotSegmentInPath {
                segment: "..".to_string(),
            },
            RequestRejection::DuplicateCookie {
                name: "session".to_string(),
            },
            RequestRejection::DuplicateFormField {
                name: "tag".to_string(),
            },
            RequestRejection::DuplicateQueryParameter {
                name: "id".to_string(),
            },
            RequestRejection::DuplicateUploadField {
                name: "avatar".to_string(),
            },
            RequestRejection::EmptyPathSegment,
            RequestRejection::EncodedPathSeparator {
                segment: "a%2Fb".to_string(),
            },
            RequestRejection::HeaderValueNotVisibleAscii {
                name: COOKIE,
                source: HeaderValue::from_bytes(&[0xC0])
                    .expect("a raw header value")
                    .to_str()
                    .expect_err("raw bytes are not visible ASCII"),
            },
            RequestRejection::MalformedContentType {
                source: "not/a/media/type"
                    .parse::<mime::Mime>()
                    .expect_err("the media type is malformed"),
            },
            RequestRejection::MalformedCookie {
                source: cookie::Cookie::parse("=nameless").expect_err("the cookie is malformed"),
            },
            RequestRejection::MalformedHost {
                source: "not a host"
                    .parse::<Authority>()
                    .expect_err("the authority is malformed"),
            },
            RequestRejection::MalformedJson {
                source: serde_json::from_slice::<serde_json::Value>(b"{not json")
                    .expect_err("the json is malformed"),
            },
            RequestRejection::MalformedMultipart {
                source: multer::Error::IncompleteStream,
            },
            RequestRejection::MalformedPercentEncoding {
                segment: "%zz".to_string(),
            },
            RequestRejection::MissingHost,
            RequestRejection::MissingMultipartBoundary,
            RequestRejection::NamelessMultipartField,
            RequestRejection::PathSegmentNotUtf8 {
                segment: "%FF".to_string(),
                source: percent_decode_str("%FF")
                    .decode_utf8()
                    .expect_err("a lone 0xFF byte is not UTF-8"),
            },
            RequestRejection::PayloadTooLarge { limit: 8 },
            RequestRejection::RepeatedSingletonHeader {
                header: SingletonRequestHeader::Cookie,
            },
            RequestRejection::RequestTargetNotOriginForm,
            RequestRejection::UnreadableBody {
                source: Box::new(std::io::Error::other("the connection closed")),
            },
            RequestRejection::UploadsDisabled,
        ]
    }

    #[test]
    fn describes_every_rejection() {
        for rejection in every_rejection() {
            assert!(
                !rejection.to_string().is_empty(),
                "{rejection:?} describes itself"
            );
        }
    }

    #[test]
    fn answers_an_oversized_payload_with_payload_too_large() {
        assert_eq!(
            RequestRejection::PayloadTooLarge { limit: 8 }
                .into_response()
                .status(),
            413
        );
    }

    #[test]
    fn answers_every_other_rejection_with_bad_request() {
        for rejection in every_rejection() {
            if matches!(rejection, RequestRejection::PayloadTooLarge { .. }) {
                continue;
            }

            assert_eq!(rejection.into_response().status(), 400);
        }
    }
}
