use bytes::Bytes;
use http_body_util::BodyExt;

use crate::body_limit::BodyLimit;
use crate::body_reading::BodyReading;
use crate::read_failure_rejection::read_failure_rejection;
use crate::request_body::RequestBody;

pub(crate) async fn collect_body(body: RequestBody, limit: BodyLimit) -> BodyReading<Bytes> {
    let limited = match body.limited(limit) {
        BodyReading::Read(limited) => limited,
        BodyReading::Rejected(rejection) => return BodyReading::Rejected(rejection),
    };

    match limited.collect().await {
        Ok(collected) => BodyReading::Read(collected.to_bytes()),
        Err(source) => BodyReading::Rejected(read_failure_rejection(source, limit)),
    }
}
