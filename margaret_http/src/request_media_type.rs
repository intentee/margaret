use http::header::CONTENT_TYPE;
use mime::Mime;

use crate::body_reading::BodyReading;
use crate::body_rejection::BodyRejection;
use crate::request::Request;

pub(crate) fn request_media_type(request: &Request) -> BodyReading<Mime> {
    let Some(value) = request.inputs.server.header(&CONTENT_TYPE) else {
        return BodyReading::Rejected(BodyRejection::MissingContentType);
    };

    match value.parse::<Mime>() {
        Ok(media_type) => BodyReading::Read(media_type),
        Err(source) => BodyReading::Rejected(BodyRejection::MalformedContentType { source }),
    }
}
