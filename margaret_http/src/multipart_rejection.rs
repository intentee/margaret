use multer::Error;

use crate::body_limit::BodyLimit;
use crate::body_rejection::BodyRejection;
use crate::read_failure_rejection::read_failure_rejection;

pub(crate) fn multipart_rejection(source: Error, limit: BodyLimit) -> BodyRejection {
    match source {
        Error::StreamReadFailed(failure) => read_failure_rejection(failure, limit),
        source => BodyRejection::MalformedMultipart { source },
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;
    use http_body_util::BodyExt;
    use http_body_util::Full;
    use http_body_util::Limited;
    use multer::Error;

    use super::multipart_rejection;
    use crate::body_limit::BodyLimit;
    use crate::body_rejection::BodyRejection;

    #[tokio::test]
    async fn reports_an_exceeded_limit_while_streaming_as_payload_too_large() {
        let exceeded = Limited::new(Full::new(Bytes::from_static(b"0123456789")), 8)
            .collect()
            .await
            .expect_err("the body exceeds its limit");

        assert!(matches!(
            multipart_rejection(
                Error::StreamReadFailed(exceeded),
                BodyLimit::new(8)
            ),
            BodyRejection::PayloadTooLarge { limit } if limit == 8
        ));
    }
}
