use percent_encoding::percent_decode_str;

use crate::request_outcome::RequestOutcome;
use crate::request_rejection::RequestRejection;

const PATH_SEPARATOR: char = '/';
const CURRENT_DIRECTORY_SEGMENT: &str = ".";
const PARENT_DIRECTORY_SEGMENT: &str = "..";

fn has_malformed_percent_escape(segment: &str) -> bool {
    let bytes = segment.as_bytes();
    let mut index = 0;

    while index < bytes.len() {
        if bytes[index] == b'%' {
            match (bytes.get(index + 1), bytes.get(index + 2)) {
                (Some(high), Some(low)) if high.is_ascii_hexdigit() && low.is_ascii_hexdigit() => {
                    index += 3;
                }
                _ => return true,
            }
        } else {
            index += 1;
        }
    }

    false
}

fn decode_segment(segment: &str) -> RequestOutcome<String> {
    if has_malformed_percent_escape(segment) {
        return RequestOutcome::Rejected(RequestRejection::MalformedPercentEncoding {
            segment: segment.to_string(),
        });
    }

    let decoded = match percent_decode_str(segment).decode_utf8() {
        Ok(decoded) => decoded.into_owned(),
        Err(source) => {
            return RequestOutcome::Rejected(RequestRejection::PathSegmentNotUtf8 {
                segment: segment.to_string(),
                source,
            });
        }
    };

    if decoded.contains(PATH_SEPARATOR) {
        return RequestOutcome::Rejected(RequestRejection::EncodedPathSeparator {
            segment: segment.to_string(),
        });
    }

    if decoded.chars().any(char::is_control) {
        return RequestOutcome::Rejected(RequestRejection::ControlCharacterInPathSegment {
            segment: segment.to_string(),
        });
    }

    if decoded == CURRENT_DIRECTORY_SEGMENT || decoded == PARENT_DIRECTORY_SEGMENT {
        return RequestOutcome::Rejected(RequestRejection::DotSegmentInPath {
            segment: segment.to_string(),
        });
    }

    RequestOutcome::Parsed(decoded)
}

pub(crate) struct RequestPath {
    decoded: String,
}

impl RequestPath {
    pub(crate) fn from_request_target(target: &str) -> RequestOutcome<Self> {
        if !target.starts_with(PATH_SEPARATOR) {
            return RequestOutcome::Rejected(RequestRejection::RequestTargetNotOriginForm);
        }

        if target.contains("//") {
            return RequestOutcome::Rejected(RequestRejection::EmptyPathSegment);
        }

        let mut decoded = String::with_capacity(target.len());

        for segment in target
            .split(PATH_SEPARATOR)
            .filter(|segment| !segment.is_empty())
        {
            match decode_segment(segment) {
                RequestOutcome::Parsed(decoded_segment) => {
                    decoded.push(PATH_SEPARATOR);
                    decoded.push_str(&decoded_segment);
                }
                RequestOutcome::Rejected(rejection) => {
                    return RequestOutcome::Rejected(rejection);
                }
            }
        }

        if decoded.is_empty() || (target.len() > 1 && target.ends_with(PATH_SEPARATOR)) {
            decoded.push(PATH_SEPARATOR);
        }

        RequestOutcome::Parsed(Self { decoded })
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.decoded
    }
}

#[cfg(test)]
mod tests {
    use std::mem::discriminant;

    use percent_encoding::percent_decode_str;

    use super::RequestPath;
    use crate::request_outcome::RequestOutcome;
    use crate::request_rejection::RequestRejection;

    fn outcome(target: &str) -> Result<String, RequestRejection> {
        match RequestPath::from_request_target(target) {
            RequestOutcome::Parsed(path) => Ok(path.as_str().to_string()),
            RequestOutcome::Rejected(rejection) => Err(rejection),
        }
    }

    fn parsed(target: &str) -> String {
        outcome(target).expect("the request target is unambiguous")
    }

    fn assert_rejects(target: &str, expected: &RequestRejection) {
        assert_eq!(
            discriminant(&outcome(target).expect_err("the request target is ambiguous")),
            discriminant(expected)
        );
    }

    fn any_segment() -> String {
        "segment".to_string()
    }

    #[test]
    fn keeps_an_unencoded_path_as_it_is() {
        assert_eq!(parsed("/articles/42"), "/articles/42");
    }

    #[test]
    fn keeps_the_root_path() {
        assert_eq!(parsed("/"), "/");
    }

    #[test]
    fn preserves_a_trailing_slash() {
        assert_eq!(parsed("/articles/"), "/articles/");
    }

    #[test]
    fn decodes_a_percent_encoded_segment() {
        assert_eq!(parsed("/files/a%20b"), "/files/a b");
    }

    #[test]
    fn decodes_a_percent_encoded_segment_exactly_once() {
        assert_eq!(parsed("/files/%2525"), "/files/%25");
    }

    #[test]
    fn rejects_a_target_that_is_not_origin_form() {
        assert_rejects("*", &RequestRejection::RequestTargetNotOriginForm);
    }

    #[test]
    fn rejects_an_empty_interior_segment() {
        assert_rejects("/articles//42", &RequestRejection::EmptyPathSegment);
    }

    #[test]
    fn rejects_a_parent_directory_segment() {
        assert_rejects(
            "/articles/../admin",
            &RequestRejection::DotSegmentInPath {
                segment: any_segment(),
            },
        );
    }

    #[test]
    fn rejects_a_current_directory_segment() {
        assert_rejects(
            "/articles/./42",
            &RequestRejection::DotSegmentInPath {
                segment: any_segment(),
            },
        );
    }

    #[test]
    fn rejects_a_percent_encoded_parent_directory_segment() {
        assert_rejects(
            "/articles/%2e%2e/admin",
            &RequestRejection::DotSegmentInPath {
                segment: any_segment(),
            },
        );
    }

    #[test]
    fn rejects_a_percent_encoded_path_separator() {
        assert_rejects(
            "/articles/a%2Fb",
            &RequestRejection::EncodedPathSeparator {
                segment: any_segment(),
            },
        );
    }

    #[test]
    fn rejects_a_malformed_percent_escape() {
        assert_rejects(
            "/articles/%zz",
            &RequestRejection::MalformedPercentEncoding {
                segment: any_segment(),
            },
        );
    }

    #[test]
    fn rejects_a_truncated_percent_escape() {
        assert_rejects(
            "/articles/%4",
            &RequestRejection::MalformedPercentEncoding {
                segment: any_segment(),
            },
        );
    }

    #[test]
    fn rejects_a_segment_that_does_not_decode_to_utf8() {
        assert_rejects(
            "/articles/%FF",
            &RequestRejection::PathSegmentNotUtf8 {
                segment: any_segment(),
                source: percent_decode_str("%FF")
                    .decode_utf8()
                    .expect_err("a lone 0xFF byte is not UTF-8"),
            },
        );
    }

    #[test]
    fn rejects_a_control_character_in_a_segment() {
        assert_rejects(
            "/articles/a%00b",
            &RequestRejection::ControlCharacterInPathSegment {
                segment: any_segment(),
            },
        );
    }

    #[test]
    fn rejects_a_carriage_return_in_a_segment() {
        assert_rejects(
            "/articles/a%0D%0Ab",
            &RequestRejection::ControlCharacterInPathSegment {
                segment: any_segment(),
            },
        );
    }
}
