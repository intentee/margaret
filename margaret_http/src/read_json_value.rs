use serde_json::Value;

use crate::body_limit::BodyLimit;
use crate::body_reading::BodyReading;
use crate::body_rejection::BodyRejection;
use crate::collect_body::collect_body;
use crate::media_type_class::MediaTypeClass;
use crate::request::Request;
use crate::request_body::RequestBody;
use crate::request_media_type::request_media_type;

pub async fn read_json_value(
    request: &Request,
    body: RequestBody,
    limit: BodyLimit,
) -> BodyReading<Value> {
    let media_type = match request_media_type(request) {
        BodyReading::Read(media_type) => media_type,
        BodyReading::Rejected(rejection) => return BodyReading::Rejected(rejection),
    };

    if MediaTypeClass::of(&media_type) != MediaTypeClass::Json {
        return BodyReading::Rejected(BodyRejection::UnsupportedMediaType { media_type });
    }

    let bytes = match collect_body(body, limit).await {
        BodyReading::Read(bytes) => bytes,
        BodyReading::Rejected(rejection) => return BodyReading::Rejected(rejection),
    };

    match serde_json::from_slice(&bytes) {
        Ok(value) => BodyReading::Read(value),
        Err(source) => BodyReading::Rejected(BodyRejection::MalformedJson { source }),
    }
}
