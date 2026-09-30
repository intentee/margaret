use std::io::Error;

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

#[tokio::test]
async fn reports_an_interrupted_raw_body_as_unreadable() {
    let body = RequestBody::new(StreamBody::new(stream::iter([Err::<Frame<Bytes>, _>(
        Error::other("the connection closed"),
    )])));
    let BodyReading::Read(mut stream) = RequestBodyStream::open(body, BodyLimit::new(64)) else {
        panic!("a raw body of unknown length opens");
    };

    assert!(matches!(
        stream.next_chunk().await,
        RequestBodyChunk::Rejected(BodyRejection::UnreadableBody { .. })
    ));
}
