use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;
use serde_json::Value;

pub struct JwtParts {
    pub header: Value,
    pub payload: Value,
}

impl JwtParts {
    pub fn of(token: &Value) -> Self {
        let mut segments = token
            .as_str()
            .expect("the token is a string")
            .split('.')
            .map(|segment| {
                serde_json::from_slice(
                    &Base64UrlUnpadded::decode_vec(segment).expect("the segment is base64url"),
                )
            });

        Self {
            header: segments
                .next()
                .expect("the token has a header")
                .expect("the header is json"),
            payload: segments
                .next()
                .expect("the token has a payload")
                .expect("the payload is json"),
        }
    }
}
