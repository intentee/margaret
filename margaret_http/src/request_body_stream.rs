use std::io;

use bytes::Bytes;
use http_body_util::BodyExt;
use http_body_util::Limited;
use http_body_util::combinators::UnsyncBoxBody;

use crate::body_limit::BodyLimit;
use crate::body_reading::BodyReading;
use crate::read_failure_rejection::read_failure_rejection;
use crate::request_body::RequestBody;
use crate::request_body_chunk::RequestBodyChunk;

pub struct RequestBodyStream {
    body: Limited<UnsyncBoxBody<Bytes, io::Error>>,
    limit: BodyLimit,
}

impl RequestBodyStream {
    #[must_use]
    pub fn open(body: RequestBody, limit: BodyLimit) -> BodyReading<Self> {
        match body.limited(limit) {
            BodyReading::Read(body) => BodyReading::Read(Self { body, limit }),
            BodyReading::Rejected(rejection) => BodyReading::Rejected(rejection),
        }
    }

    pub async fn next_chunk(&mut self) -> RequestBodyChunk {
        loop {
            match self.body.frame().await {
                Some(Ok(frame)) => {
                    if let Ok(data) = frame.into_data() {
                        return RequestBodyChunk::Data(data);
                    }
                }
                Some(Err(source)) => {
                    return RequestBodyChunk::Rejected(read_failure_rejection(source, self.limit));
                }
                None => return RequestBodyChunk::End,
            }
        }
    }
}
