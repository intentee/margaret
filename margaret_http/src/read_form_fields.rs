use std::collections::HashMap;

use mime::Mime;

use crate::body_limit::BodyLimit;
use crate::body_reading::BodyReading;
use crate::body_rejection::BodyRejection;
use crate::collect_body::collect_body;
use crate::form_fields::form_fields;
use crate::index_form_fields::index_form_fields;
use crate::media_type_class::MediaTypeClass;
use crate::multipart_part::MultipartPart;
use crate::multipart_parts::MultipartParts;
use crate::request::Request;
use crate::request_body::RequestBody;
use crate::request_media_type::request_media_type;

async fn read_urlencoded_fields(
    body: RequestBody,
    limit: BodyLimit,
) -> BodyReading<HashMap<String, String>> {
    match collect_body(body, limit).await {
        BodyReading::Read(bytes) => index_form_fields(form_fields(&bytes)),
        BodyReading::Rejected(rejection) => BodyReading::Rejected(rejection),
    }
}

async fn read_multipart_fields(
    media_type: &Mime,
    body: RequestBody,
    limit: BodyLimit,
) -> BodyReading<HashMap<String, String>> {
    let mut parts = match MultipartParts::open(media_type, body, limit) {
        BodyReading::Read(parts) => parts,
        BodyReading::Rejected(rejection) => return BodyReading::Rejected(rejection),
    };
    let mut fields = Vec::new();

    loop {
        match parts.next_part().await {
            BodyReading::Read(MultipartPart::Text(field)) => fields.push(field),
            BodyReading::Read(MultipartPart::File(part)) => {
                return BodyReading::Rejected(BodyRejection::UnexpectedUploadField {
                    name: part.name,
                });
            }
            BodyReading::Read(MultipartPart::End) => return index_form_fields(fields),
            BodyReading::Rejected(rejection) => return BodyReading::Rejected(rejection),
        }
    }
}

pub async fn read_form_fields(
    request: &Request,
    body: RequestBody,
    limit: BodyLimit,
) -> BodyReading<HashMap<String, String>> {
    let media_type = match request_media_type(request) {
        BodyReading::Read(media_type) => media_type,
        BodyReading::Rejected(rejection) => return BodyReading::Rejected(rejection),
    };

    match MediaTypeClass::of(&media_type) {
        MediaTypeClass::UrlEncodedForm => read_urlencoded_fields(body, limit).await,
        MediaTypeClass::MultipartFormData => read_multipart_fields(&media_type, body, limit).await,
        MediaTypeClass::Json | MediaTypeClass::Other => {
            BodyReading::Rejected(BodyRejection::UnsupportedMediaType { media_type })
        }
    }
}
