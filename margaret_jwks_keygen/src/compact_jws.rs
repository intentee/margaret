use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;

use crate::token_malformation::TokenMalformation;

pub(crate) struct CompactJws<'token> {
    pub(crate) header_bytes: Vec<u8>,
    pub(crate) payload_bytes: Vec<u8>,
    pub(crate) signature_bytes: Vec<u8>,
    pub(crate) signing_input: &'token str,
}

impl<'token> CompactJws<'token> {
    pub(crate) fn parse(token: &'token str) -> Result<Self, TokenMalformation> {
        let (signing_input, signature_segment) = token
            .rsplit_once('.')
            .ok_or(TokenMalformation::NotCompactJws)?;
        let (header_segment, payload_segment) = signing_input
            .split_once('.')
            .ok_or(TokenMalformation::NotCompactJws)?;
        let header_bytes = Base64UrlUnpadded::decode_vec(header_segment)
            .map_err(TokenMalformation::HeaderBase64)?;
        let payload_bytes = Base64UrlUnpadded::decode_vec(payload_segment)
            .map_err(TokenMalformation::ClaimsBase64)?;
        let signature_bytes = Base64UrlUnpadded::decode_vec(signature_segment)
            .map_err(TokenMalformation::SignatureBase64)?;

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

    fn parse_failure(token: &str) -> Option<String> {
        match CompactJws::parse(token) {
            Ok(_) => None,
            Err(malformation) => Some(malformation.to_string()),
        }
    }

    #[test]
    fn parses_the_three_segments() {
        let compact = CompactJws::parse("aGVhZGVy.cGF5bG9hZA.AAAA").expect("the token parses");

        assert_eq!(compact.signing_input, "aGVhZGVy.cGF5bG9hZA");
        assert_eq!(compact.header_bytes, b"header");
        assert_eq!(compact.payload_bytes, b"payload");
        assert_eq!(
            compact.signature_bytes,
            Base64UrlUnpadded::decode_vec("AAAA").expect("the fixture decodes")
        );
        assert_eq!(parse_failure("aGVhZGVy.cGF5bG9hZA.AAAA"), None);
    }

    #[test]
    fn reports_a_token_that_has_no_dot() {
        assert_eq!(
            parse_failure("single").as_deref(),
            Some("the token is not a well-formed compact jws")
        );
    }

    #[test]
    fn reports_a_token_that_has_only_two_segments() {
        assert_eq!(
            parse_failure("header.signature").as_deref(),
            Some("the token is not a well-formed compact jws")
        );
    }

    #[test]
    fn reports_a_header_segment_that_is_not_base64url() {
        assert_eq!(
            parse_failure("!!!.cGF5bG9hZA.AAAA"),
            Some(format!(
                "the token header segment is not valid base64url: {}",
                Base64UrlUnpadded::decode_vec("!!!").expect_err("the fixture is not base64url")
            ))
        );
    }

    #[test]
    fn reports_a_payload_segment_that_is_not_base64url() {
        assert_eq!(
            parse_failure("aGVhZGVy.!!!.AAAA"),
            Some(format!(
                "the token payload segment is not valid base64url: {}",
                Base64UrlUnpadded::decode_vec("!!!").expect_err("the fixture is not base64url")
            ))
        );
    }

    #[test]
    fn reports_a_signature_segment_that_is_not_base64url() {
        assert_eq!(
            parse_failure("aGVhZGVy.cGF5bG9hZA.!!!"),
            Some(format!(
                "the token signature segment is not valid base64url: {}",
                Base64UrlUnpadded::decode_vec("!!!").expect_err("the fixture is not base64url")
            ))
        );
    }
}
