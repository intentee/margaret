use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::str::Utf8Error;

use cookie::ParseError;
use http::HeaderName;
use http::header::ToStrError;
use http::uri::InvalidUri;

use crate::singleton_request_header::SingletonRequestHeader;

#[derive(Debug)]
pub enum RequestRejection {
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
    DuplicateQueryParameter {
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
    MalformedCookie {
        source: ParseError,
    },
    MalformedHost {
        source: InvalidUri,
    },
    MalformedPercentEncoding {
        segment: String,
    },
    MissingHost,
    PathSegmentNotUtf8 {
        segment: String,
        source: Utf8Error,
    },
    RepeatedSingletonHeader {
        header: SingletonRequestHeader,
    },
    RequestTargetNotOriginForm,
}

impl Display for RequestRejection {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
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
            Self::DuplicateQueryParameter { name } => {
                write!(formatter, "the query string repeats the parameter '{name}'")
            }
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
            Self::MalformedCookie { source } => {
                write!(formatter, "a request cookie could not be parsed: {source}")
            }
            Self::MalformedHost { source } => write!(
                formatter,
                "the Host header is not a valid authority: {source}"
            ),
            Self::MalformedPercentEncoding { segment } => write!(
                formatter,
                "the path segment '{segment}' is not validly percent encoded"
            ),
            Self::MissingHost => formatter.write_str("the request has no Host header"),
            Self::PathSegmentNotUtf8 { segment, source } => write!(
                formatter,
                "the path segment '{segment}' does not decode to UTF-8: {source}"
            ),
            Self::RepeatedSingletonHeader { header } => write!(
                formatter,
                "the request repeats the single valued header {header:?}"
            ),
            Self::RequestTargetNotOriginForm => {
                formatter.write_str("the request target is not in origin form")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use cookie::Cookie;
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
            RequestRejection::DuplicateQueryParameter {
                name: "id".to_string(),
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
            RequestRejection::MalformedCookie {
                source: Cookie::parse("=nameless").expect_err("the cookie is malformed"),
            },
            RequestRejection::MalformedHost {
                source: "not a host"
                    .parse::<Authority>()
                    .expect_err("the authority is malformed"),
            },
            RequestRejection::MalformedPercentEncoding {
                segment: "%zz".to_string(),
            },
            RequestRejection::MissingHost,
            RequestRejection::PathSegmentNotUtf8 {
                segment: "%FF".to_string(),
                source: percent_decode_str("%FF")
                    .decode_utf8()
                    .expect_err("a lone 0xFF byte is not UTF-8"),
            },
            RequestRejection::RepeatedSingletonHeader {
                header: SingletonRequestHeader::Cookie,
            },
            RequestRejection::RequestTargetNotOriginForm,
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
}
