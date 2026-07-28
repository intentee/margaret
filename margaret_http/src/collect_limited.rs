use bytes::Bytes;
use http_body_util::BodyExt;
use http_body_util::LengthLimitError;
use http_body_util::Limited;

use crate::request_body::RequestBody;
use crate::request_outcome::RequestOutcome;
use crate::request_rejection::RequestRejection;

pub(crate) async fn collect_limited(body: RequestBody, max_size: u64) -> RequestOutcome<Bytes> {
    match Limited::new(body, max_size as usize).collect().await {
        Ok(collected) => RequestOutcome::Parsed(collected.to_bytes()),
        Err(source) => match source.downcast::<LengthLimitError>() {
            Ok(_) => {
                RequestOutcome::Rejected(RequestRejection::PayloadTooLarge { limit: max_size })
            }
            Err(source) => RequestOutcome::Rejected(RequestRejection::UnreadableBody { source }),
        },
    }
}
