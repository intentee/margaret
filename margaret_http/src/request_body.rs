use std::error::Error;
use std::io;

use bytes::Bytes;
use http_body::Body;
use http_body_util::BodyExt;
use http_body_util::Limited;
use http_body_util::combinators::UnsyncBoxBody;

use crate::body_limit::BodyLimit;
use crate::body_reading::BodyReading;
use crate::body_rejection::BodyRejection;
use crate::stall_limited_body::StallLimitedBody;

pub struct RequestBody {
    body: UnsyncBoxBody<Bytes, io::Error>,
}

impl RequestBody {
    #[must_use]
    pub fn new<Source>(source: Source) -> Self
    where
        Source: Body<Data = Bytes> + Send + 'static,
        Source::Error: Into<Box<dyn Error + Send + Sync>>,
    {
        Self {
            body: StallLimitedBody::new(source.map_err(io::Error::other).boxed_unsync())
                .boxed_unsync(),
        }
    }

    pub(crate) fn limited(
        self,
        limit: BodyLimit,
    ) -> BodyReading<Limited<UnsyncBoxBody<Bytes, io::Error>>> {
        if limit.admits(self.body.size_hint().lower()) {
            BodyReading::Read(Limited::new(self.body, limit.max_bytes()))
        } else {
            BodyReading::Rejected(BodyRejection::PayloadTooLarge {
                limit: limit.max_bytes(),
            })
        }
    }
}
