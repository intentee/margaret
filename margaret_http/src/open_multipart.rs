use crate::body_limit::BodyLimit;
use crate::body_reading::BodyReading;
use crate::body_rejection::BodyRejection;
use crate::media_type_class::MediaTypeClass;
use crate::multipart_parts::MultipartParts;
use crate::request::Request;
use crate::request_body::RequestBody;
use crate::request_media_type::request_media_type;

pub(crate) fn open_multipart(
    request: &Request,
    body: RequestBody,
    limit: BodyLimit,
) -> BodyReading<MultipartParts> {
    let media_type = match request_media_type(request) {
        BodyReading::Read(media_type) => media_type,
        BodyReading::Rejected(rejection) => return BodyReading::Rejected(rejection),
    };

    if MediaTypeClass::of(&media_type) == MediaTypeClass::MultipartFormData {
        MultipartParts::open(&media_type, body, limit)
    } else {
        BodyReading::Rejected(BodyRejection::UnsupportedMediaType { media_type })
    }
}
