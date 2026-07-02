use http::HeaderMap;
use http::header::CONTENT_TYPE;

use crate::request_error::RequestError;

fn media_type(headers: &HeaderMap) -> Option<mime::Mime> {
    headers
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<mime::Mime>().ok())
}

fn is_multipart(media: &mime::Mime) -> bool {
    media.type_() == mime::MULTIPART && media.subtype() == mime::FORM_DATA
}

fn is_urlencoded(media: &mime::Mime) -> bool {
    media.type_() == mime::APPLICATION && media.subtype() == mime::WWW_FORM_URLENCODED
}

fn is_json(media: &mime::Mime) -> bool {
    media.type_() == mime::APPLICATION && media.subtype() == mime::JSON
}

pub(crate) enum BodyClass {
    Multipart { boundary: String },
    UrlEncoded,
    Json,
    Other,
}

impl BodyClass {
    pub(crate) fn from_headers(headers: &HeaderMap) -> Result<Self, RequestError> {
        let Some(media) = media_type(headers) else {
            return Ok(Self::Other);
        };

        if is_multipart(&media) {
            let boundary = media
                .get_param(mime::BOUNDARY)
                .ok_or(RequestError::MissingMultipartBoundary)?
                .as_str()
                .to_string();

            Ok(Self::Multipart { boundary })
        } else if is_urlencoded(&media) {
            Ok(Self::UrlEncoded)
        } else if is_json(&media) {
            Ok(Self::Json)
        } else {
            Ok(Self::Other)
        }
    }
}
