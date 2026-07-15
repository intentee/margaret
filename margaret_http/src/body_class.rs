use http::HeaderMap;
use http::header::CONTENT_TYPE;

use crate::request_error::RequestError;

fn media_type(headers: &HeaderMap) -> Result<Option<mime::Mime>, RequestError> {
    let Some(value) = headers.get(CONTENT_TYPE) else {
        return Ok(None);
    };

    let media = value
        .to_str()?
        .parse::<mime::Mime>()
        .map_err(|source| RequestError::MalformedContentType { source })?;

    Ok(Some(media))
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
        let Some(media) = media_type(headers)? else {
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

#[cfg(test)]
mod tests {
    use http::HeaderMap;
    use http::HeaderValue;
    use http::header::CONTENT_TYPE;

    use super::BodyClass;

    fn headers(content_type: HeaderValue) -> HeaderMap {
        let mut headers = HeaderMap::new();

        headers.insert(CONTENT_TYPE, content_type);

        headers
    }

    #[test]
    fn reports_a_content_type_that_is_not_a_media_type() {
        let result = BodyClass::from_headers(&headers(HeaderValue::from_static("not/a/media/type")));

        assert!(result.is_err());
    }

    #[test]
    fn reports_a_content_type_that_is_not_visible_ascii() {
        let result = BodyClass::from_headers(&headers(
            HeaderValue::from_bytes(b"text/plain; charset=\xff")
                .expect("the header value carries raw bytes"),
        ));

        assert!(result.is_err());
    }
}
