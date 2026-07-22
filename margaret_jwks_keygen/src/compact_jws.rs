use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;

use crate::jwks_key_error::JwksKeyError;

pub(crate) struct CompactJws<'token> {
    pub(crate) header_bytes: Vec<u8>,
    pub(crate) payload_bytes: Vec<u8>,
    pub(crate) signature_bytes: Vec<u8>,
    pub(crate) signing_input: &'token str,
}

impl<'token> CompactJws<'token> {
    pub(crate) fn parse(token: &'token str) -> Result<Self, JwksKeyError> {
        let (signing_input, signature_segment) = token
            .rsplit_once('.')
            .ok_or(JwksKeyError::MalformedCompactJws)?;
        let (header_segment, payload_segment) = signing_input
            .split_once('.')
            .ok_or(JwksKeyError::MalformedCompactJws)?;
        let header_bytes = Base64UrlUnpadded::decode_vec(header_segment)
            .map_err(|source| JwksKeyError::HeaderBase64 { source })?;
        let payload_bytes = Base64UrlUnpadded::decode_vec(payload_segment)
            .map_err(|source| JwksKeyError::ClaimsBase64 { source })?;
        let signature_bytes = Base64UrlUnpadded::decode_vec(signature_segment)
            .map_err(|source| JwksKeyError::SignatureBase64 { source })?;

        Ok(Self {
            header_bytes,
            payload_bytes,
            signature_bytes,
            signing_input,
        })
    }
}

#[cfg(test)]
mod tests {
    use base64ct::Base64UrlUnpadded;
    use base64ct::Encoding;

    use super::CompactJws;

    #[test]
    fn parses_the_three_segments() {
        let compact = CompactJws::parse("aGVhZGVy.cGF5bG9hZA.AAAA").unwrap();

        assert_eq!(compact.signing_input, "aGVhZGVy.cGF5bG9hZA");
        assert_eq!(compact.header_bytes, b"header");
        assert_eq!(compact.payload_bytes, b"payload");
        assert_eq!(
            compact.signature_bytes,
            Base64UrlUnpadded::decode_vec("AAAA").unwrap()
        );
    }

    #[test]
    fn errors_when_token_has_no_dot() {
        assert_eq!(
            CompactJws::parse("single").err().unwrap().to_string(),
            "the token is not a well-formed compact jws"
        );
    }

    #[test]
    fn errors_when_token_has_only_two_segments() {
        assert_eq!(
            CompactJws::parse("header.signature")
                .err()
                .unwrap()
                .to_string(),
            "the token is not a well-formed compact jws"
        );
    }

    #[test]
    fn errors_when_header_segment_is_not_base64url() {
        assert_eq!(
            CompactJws::parse("!!!.cGF5bG9hZA.AAAA")
                .err()
                .unwrap()
                .to_string(),
            format!(
                "the token header segment is not valid base64url: {}",
                Base64UrlUnpadded::decode_vec("!!!").err().unwrap()
            )
        );
    }

    #[test]
    fn errors_when_payload_segment_is_not_base64url() {
        assert_eq!(
            CompactJws::parse("aGVhZGVy.!!!.AAAA")
                .err()
                .unwrap()
                .to_string(),
            format!(
                "the token payload segment is not valid base64url: {}",
                Base64UrlUnpadded::decode_vec("!!!").err().unwrap()
            )
        );
    }

    #[test]
    fn errors_when_signature_segment_is_not_base64url() {
        assert_eq!(
            CompactJws::parse("aGVhZGVy.cGF5bG9hZA.!!!")
                .err()
                .unwrap()
                .to_string(),
            format!(
                "the token signature segment is not valid base64url: {}",
                Base64UrlUnpadded::decode_vec("!!!").err().unwrap()
            )
        );
    }
}
