use std::convert::Infallible;

use bytes::Bytes;
use futures_util::stream;
use http_body::Frame;
use http_body_util::StreamBody;

use margaret_http::body_limit::BodyLimit;
use margaret_http::body_reading::BodyReading;
use margaret_http::body_rejection::BodyRejection;
use margaret_http::request_body::RequestBody;
use margaret_http::request_body_chunk::RequestBodyChunk;
use margaret_http::request_body_stream::RequestBodyStream;

#[tokio::test(start_paused = true)]
async fn rejects_a_raw_body_that_stalls() {
    let body = RequestBody::new(StreamBody::new(stream::pending::<
        Result<Frame<Bytes>, Infallible>,
    >()));
    let BodyReading::Read(mut stream) = RequestBodyStream::open(body, BodyLimit::new(4)) else {
        panic!("a raw body of unknown length opens");
    };

    assert!(matches!(
        stream.next_chunk().await,
        RequestBodyChunk::Rejected(BodyRejection::Stalled)
    ));
}
