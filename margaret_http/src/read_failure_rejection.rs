use std::error::Error;
use std::io;

use http_body_util::LengthLimitError;

use crate::body_limit::BodyLimit;
use crate::body_rejection::BodyRejection;

pub(crate) fn read_failure_rejection(
    source: Box<dyn Error + Send + Sync>,
    limit: BodyLimit,
) -> BodyRejection {
    if source.is::<LengthLimitError>() {
        BodyRejection::PayloadTooLarge {
            limit: limit.max_bytes(),
        }
    } else if source
        .downcast_ref::<io::Error>()
        .is_some_and(|failure| failure.kind() == io::ErrorKind::TimedOut)
    {
        BodyRejection::Stalled
    } else {
        BodyRejection::UnreadableBody { source }
    }
}

#[cfg(test)]
mod tests {
    use std::io;

    use bytes::Bytes;
    use http_body_util::BodyExt;
    use http_body_util::Full;
    use http_body_util::Limited;

    use super::read_failure_rejection;
    use crate::body_limit::BodyLimit;
    use crate::body_rejection::BodyRejection;

    #[tokio::test]
    async fn reports_an_exceeded_limit_as_payload_too_large() {
        let exceeded = Limited::new(Full::new(Bytes::from_static(b"0123456789")), 8)
            .collect()
            .await
            .expect_err("the body exceeds its limit");

        assert!(matches!(
            read_failure_rejection(exceeded, BodyLimit::new(8)),
            BodyRejection::PayloadTooLarge { limit } if limit == 8
        ));
    }

    #[test]
    fn reports_a_transport_failure_as_an_unreadable_body() {
        assert!(matches!(
            read_failure_rejection(
                Box::new(io::Error::other("the connection closed")),
                BodyLimit::new(8)
            ),
            BodyRejection::UnreadableBody { source } if source.to_string() == "the connection closed"
        ));
    }
}
