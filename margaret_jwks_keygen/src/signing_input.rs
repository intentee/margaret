use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;
use serde_json::Value;

pub(crate) fn signing_input(encoded_header: &str, claims: &Value) -> String {
    format!(
        "{encoded_header}.{}",
        Base64UrlUnpadded::encode_string(claims.to_string().as_bytes()),
    )
}
