use percent_encoding::percent_decode_str;

use crate::route_parameter_decoding_outcome::RouteParameterDecodingOutcome;

pub(crate) fn decode_url_path_parameter(value: &str) -> RouteParameterDecodingOutcome {
    match percent_decode_str(value).decode_utf8() {
        Ok(decoded) => RouteParameterDecodingOutcome::Decoded(decoded.into_owned()),
        Err(_) => RouteParameterDecodingOutcome::NotUtf8,
    }
}

#[cfg(test)]
mod tests {
    use super::decode_url_path_parameter;
    use crate::encode_url_catch_all::encode_url_catch_all;
    use crate::encode_url_path_segment::encode_url_path_segment;
    use crate::route_parameter_decoding_outcome::RouteParameterDecodingOutcome;

    const AWKWARD: &str = ".a/b?c#d e%f żółw";

    fn decoded(value: &str) -> String {
        match decode_url_path_parameter(value) {
            RouteParameterDecodingOutcome::Decoded(decoded) => decoded,
            RouteParameterDecodingOutcome::NotUtf8 => String::from("not utf-8"),
        }
    }

    #[test]
    fn decodes_a_percent_encoded_value() {
        assert_eq!(decoded("..%2Fadmin"), "../admin");
    }

    #[test]
    fn reports_a_value_that_is_not_valid_utf8() {
        assert_eq!(decoded("%FF"), "not utf-8");
    }

    #[test]
    fn round_trips_a_segment_parameter() {
        assert_eq!(decoded(&encode_url_path_segment(AWKWARD)), AWKWARD);
    }

    #[test]
    fn round_trips_a_catch_all_parameter() {
        assert_eq!(decoded(&encode_url_catch_all(AWKWARD)), AWKWARD);
    }
}
