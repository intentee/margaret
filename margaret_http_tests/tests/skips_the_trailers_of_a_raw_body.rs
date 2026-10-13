use std::convert::Infallible;

use bytes::Bytes;
use futures_util::stream;
use http::HeaderMap;
use http_body::Frame;
use http_body_util::StreamBody;

use margaret_http::body_limit::BodyLimit;
use margaret_http::body_reading::BodyReading;
use margaret_http::request_body::RequestBody;
use margaret_http::request_body_chunk::RequestBodyChunk;
use margaret_http::request_body_stream::RequestBodyStream;

#[tokio::test]
async fn skips_the_trailers_of_a_raw_body() {
    let body = RequestBody::new(StreamBody::new(stream::iter([
        Ok::<_, Infallible>(Frame::data(Bytes::from_static(b"payload"))),
        Ok(Frame::trailers(HeaderMap::new())),
    ])));
    let BodyReading::Read(mut stream) = RequestBodyStream::open(body, BodyLimit::new(64)) else {
        panic!("a raw body within its limit opens");
    };

    assert!(
        matches!(stream.next_chunk().await, RequestBodyChunk::Data(chunk) if chunk == "payload")
    );
    assert!(matches!(stream.next_chunk().await, RequestBodyChunk::End));
}
