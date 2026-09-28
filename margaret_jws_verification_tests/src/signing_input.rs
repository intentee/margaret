use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;
use serde_json::Value;

#[must_use]
pub fn signing_input(header: &Value, claims: &Value) -> String {
    format!(
        "{}.{}",
        Base64UrlUnpadded::encode_string(header.to_string().as_bytes()),
        Base64UrlUnpadded::encode_string(claims.to_string().as_bytes()),
    )
}
