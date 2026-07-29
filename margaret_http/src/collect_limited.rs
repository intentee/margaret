use bytes::Bytes;
use http_body_util::BodyExt;
use http_body_util::LengthLimitError;
use http_body_util::Limited;

use crate::request_body::RequestBody;
use crate::request_error::RequestError;

pub(crate) async fn collect_limited(
    body: RequestBody,
    max_size: usize,
) -> Result<Bytes, RequestError> {
    match Limited::new(body, max_size).collect().await {
        Ok(collected) => Ok(collected.to_bytes()),
        Err(source) => match source.downcast::<LengthLimitError>() {
            Ok(_) => Err(RequestError::PayloadTooLarge {
                limit: max_size as u64,
            }),
            Err(source) => Err(RequestError::BodyRead { source }),
        },
    }
}
