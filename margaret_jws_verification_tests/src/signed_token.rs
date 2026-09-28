use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;

#[must_use]
pub fn signed_token(signing_input: &str, signature: &[u8]) -> String {
    format!(
        "{signing_input}.{}",
        Base64UrlUnpadded::encode_string(signature)
    )
}
