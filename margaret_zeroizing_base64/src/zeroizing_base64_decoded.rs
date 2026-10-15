use base64ct::Base64;
use base64ct::Encoding;
use zeroize::Zeroizing;

use crate::constants::BASE64_GROUP_BYTES;
use crate::constants::BASE64_GROUP_CHARACTERS;

/// # Errors
///
/// Returns `base64ct::Error` when the input is not standard padded base64.
pub fn zeroizing_base64_decoded(encoded: &str) -> Result<Zeroizing<Vec<u8>>, base64ct::Error> {
    let mut decoded = Zeroizing::new(vec![
        0;
        encoded.len().div_ceil(BASE64_GROUP_CHARACTERS)
            * BASE64_GROUP_BYTES
    ]);
    let length = Base64::decode(encoded, &mut decoded)?.len();

    decoded.truncate(length);

    Ok(decoded)
}

#[cfg(test)]
mod tests {
    use super::zeroizing_base64_decoded;

    #[test]
    fn decodes_exactly_the_bytes_the_input_encodes() {
        assert_eq!(
            *zeroizing_base64_decoded("c2VjcmV0").expect("the input is base64"),
            b"secret"
        );
    }

    #[test]
    fn rejects_an_input_that_is_not_standard_padded_base64() {
        assert_eq!(
            zeroizing_base64_decoded("c2VjcmV0-_").expect_err("url-safe characters are refused"),
            base64ct::Error::InvalidEncoding
        );
    }
}
