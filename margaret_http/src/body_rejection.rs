use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use mime::FromStrError;
use mime::Mime;

use crate::response::Response;

#[derive(Debug)]
pub enum BodyRejection {
    DuplicateFormField {
        name: String,
    },
    DuplicateUploadField {
        name: String,
    },
    MalformedContentType {
        source: FromStrError,
    },
    MalformedJson {
        source: serde_json::Error,
    },
    MalformedMultipart {
        source: multer::Error,
    },
    MissingContentType,
    MissingMultipartBoundary,
    NamelessMultipartField,
    PayloadTooLarge {
        limit: usize,
    },
    UnexpectedFormField {
        name: String,
    },
    UnexpectedUploadField {
        name: String,
    },
    UnreadableBody {
        source: Box<dyn Error + Send + Sync>,
    },
    UnsupportedMediaType {
        media_type: Mime,
    },
    UploadsDisabled,
}

impl BodyRejection {
    #[must_use]
    pub fn into_response(self) -> Response {
        eprintln!("margaret_http: the request body was rejected: {self}");

        match self {
            Self::PayloadTooLarge { .. } => Response::text(413, "Payload Too Large"),
            Self::MissingContentType | Self::UnsupportedMediaType { .. } => {
                Response::text(415, "Unsupported Media Type")
            }
            Self::DuplicateFormField { .. }
            | Self::DuplicateUploadField { .. }
            | Self::MalformedContentType { .. }
            | Self::MalformedJson { .. }
            | Self::MalformedMultipart { .. }
            | Self::MissingMultipartBoundary
            | Self::NamelessMultipartField
            | Self::UnexpectedFormField { .. }
            | Self::UnexpectedUploadField { .. }
            | Self::UnreadableBody { .. }
            | Self::UploadsDisabled => Response::text(400, "Bad Request"),
        }
    }
}

impl Display for BodyRejection {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::DuplicateFormField { name } => {
                write!(
                    formatter,
                    "the request body repeats the form field '{name}'"
                )
            }
            Self::DuplicateUploadField { name } => write!(
                formatter,
                "the request body repeats the upload field '{name}'"
            ),
            Self::MalformedContentType { source } => write!(
                formatter,
                "the request content type is not a valid media type: {source}"
            ),
            Self::MalformedJson { source } => {
                write!(formatter, "the request body is not valid JSON: {source}")
            }
            Self::MalformedMultipart { source } => write!(
                formatter,
                "the multipart request body could not be parsed: {source}"
            ),
            Self::MissingContentType => {
                formatter.write_str("the request body does not declare its content type")
            }
            Self::MissingMultipartBoundary => {
                formatter.write_str("the multipart request is missing its Content-Type boundary")
            }
            Self::NamelessMultipartField => formatter
                .write_str("a multipart request part is missing its Content-Disposition name"),
            Self::PayloadTooLarge { limit } => {
                write!(formatter, "the request body exceeds the {limit} byte limit")
            }
            Self::UnexpectedFormField { name } => write!(
                formatter,
                "the route does not accept the form field '{name}'"
            ),
            Self::UnexpectedUploadField { name } => write!(
                formatter,
                "the route does not accept the uploaded file '{name}'"
            ),
            Self::UnreadableBody { source } => {
                write!(formatter, "the request body could not be read: {source}")
            }
            Self::UnsupportedMediaType { media_type } => write!(
                formatter,
                "the route does not accept a '{media_type}' request body"
            ),
            Self::UploadsDisabled => formatter
                .write_str("the route accepts uploaded files but file uploads are disabled"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io;

    use mime::Mime;

    use super::BodyRejection;

    fn every_rejection() -> Vec<BodyRejection> {
        vec![
            BodyRejection::DuplicateFormField {
                name: "tag".to_string(),
            },
            BodyRejection::DuplicateUploadField {
                name: "avatar".to_string(),
            },
            BodyRejection::MalformedContentType {
                source: "not/a/media/type"
                    .parse::<Mime>()
                    .expect_err("the media type is malformed"),
            },
            BodyRejection::MalformedJson {
                source: serde_json::from_slice::<serde_json::Value>(b"{not json")
                    .expect_err("the json is malformed"),
            },
            BodyRejection::MalformedMultipart {
                source: multer::Error::IncompleteStream,
            },
            BodyRejection::MissingContentType,
            BodyRejection::MissingMultipartBoundary,
            BodyRejection::NamelessMultipartField,
            BodyRejection::PayloadTooLarge { limit: 8 },
            BodyRejection::UnexpectedFormField {
                name: "title".to_string(),
            },
            BodyRejection::UnexpectedUploadField {
                name: "cover".to_string(),
            },
            BodyRejection::UnreadableBody {
                source: Box::new(io::Error::other("the connection closed")),
            },
            BodyRejection::UnsupportedMediaType {
                media_type: mime::TEXT_PLAIN,
            },
            BodyRejection::UploadsDisabled,
        ]
    }

    fn status(rejection: BodyRejection) -> u16 {
        rejection.into_response().status()
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
    fn answers_an_oversized_body_with_payload_too_large() {
        assert_eq!(status(BodyRejection::PayloadTooLarge { limit: 8 }), 413);
    }

    #[test]
    fn answers_an_undeclared_content_type_with_unsupported_media_type() {
        assert_eq!(status(BodyRejection::MissingContentType), 415);
    }

    #[test]
    fn answers_an_unaccepted_content_type_with_unsupported_media_type() {
        assert_eq!(
            status(BodyRejection::UnsupportedMediaType {
                media_type: mime::TEXT_PLAIN,
            }),
            415
        );
    }

    #[test]
    fn answers_every_malformed_body_with_bad_request() {
        for rejection in every_rejection() {
            if matches!(
                rejection,
                BodyRejection::PayloadTooLarge { .. }
                    | BodyRejection::MissingContentType
                    | BodyRejection::UnsupportedMediaType { .. }
            ) {
                continue;
            }

            assert_eq!(status(rejection), 400);
        }
    }
}
