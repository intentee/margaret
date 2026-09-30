use bytes::BytesMut;
use reqwest::Response;

use crate::issuer_response_max_bytes::ISSUER_RESPONSE_MAX_BYTES;
use crate::response_body::ResponseBody;

pub(crate) async fn read_response_body(mut response: Response) -> ResponseBody {
    let mut body = BytesMut::new();

    loop {
        match response.chunk().await {
            Ok(Some(chunk)) if body.len() + chunk.len() > ISSUER_RESPONSE_MAX_BYTES => {
                return ResponseBody::Oversized {
                    max_bytes: ISSUER_RESPONSE_MAX_BYTES,
                };
            }
            Ok(Some(chunk)) => body.extend_from_slice(&chunk),
            Ok(None) => return ResponseBody::Read(body.freeze()),
            Err(error) => return ResponseBody::Failed(error),
        }
    }
}
