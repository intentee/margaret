use std::marker::PhantomData;

use crate::admit_path_segment::admit_path_segment;
use crate::path_segment_admission::PathSegmentAdmission;
use crate::path_segment_rejection::PathSegmentRejection;
use crate::segment_prefix::SegmentPrefix;

pub struct PathParameterValue<TPrefix> {
    prefix: PhantomData<fn() -> TPrefix>,
    value: String,
}

impl<TPrefix: SegmentPrefix> PathParameterValue<TPrefix> {
    /// # Errors
    ///
    /// Returns `PathSegmentRejection::Empty` for an empty value, and the `PathSegmentRejection` of
    /// a value that cannot fill a path segment after the literal text preceding it.
    pub fn new(value: String) -> Result<Self, PathSegmentRejection> {
        if value.is_empty() {
            return Err(PathSegmentRejection::Empty);
        }

        match admit_path_segment(&[TPrefix::TEXT, &value].concat()) {
            PathSegmentAdmission::Admitted => Ok(Self {
                prefix: PhantomData,
                value,
            }),
            PathSegmentAdmission::Rejected(rejection) => Err(rejection),
        }
    }

    #[must_use]
    pub fn into_string(self) -> String {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::PathParameterValue;
    use crate::path_segment_rejection::PathSegmentRejection;
    use crate::segment_prefix::SegmentPrefix;
    use crate::unprefixed_segment::UnprefixedSegment;

    struct DotPrefix;

    impl SegmentPrefix for DotPrefix {
        const TEXT: &'static str = ".";
    }

    struct ImagePrefix;

    impl SegmentPrefix for ImagePrefix {
        const TEXT: &'static str = "img.";
    }

    fn unprefixed(value: &str) -> Result<String, PathSegmentRejection> {
        PathParameterValue::<UnprefixedSegment>::new(value.to_string())
            .map(PathParameterValue::into_string)
    }

    #[test]
    fn keeps_a_value_that_fills_its_segment() {
        assert_eq!(unprefixed("rust"), Ok("rust".to_string()));
    }

    #[test]
    fn keeps_a_leading_dot_of_a_value_that_is_not_a_dot_segment() {
        assert_eq!(unprefixed(".env"), Ok(".env".to_string()));
    }

    #[test]
    fn refuses_a_parent_directory_value_since_clients_would_collapse_the_segment() {
        assert_eq!(unprefixed(".."), Err(PathSegmentRejection::DotSegment));
    }

    #[test]
    fn refuses_a_current_directory_value_since_clients_would_drop_the_segment() {
        assert_eq!(unprefixed("."), Err(PathSegmentRejection::DotSegment));
    }

    #[test]
    fn refuses_an_empty_value_since_an_empty_segment_cannot_address_the_route() {
        assert_eq!(unprefixed(""), Err(PathSegmentRejection::Empty));
    }

    #[test]
    fn refuses_a_value_that_contains_a_path_separator() {
        assert_eq!(
            unprefixed("rust/lang"),
            Err(PathSegmentRejection::Separator)
        );
    }

    #[test]
    fn refuses_a_value_that_contains_a_line_break() {
        assert_eq!(
            unprefixed("a\r\nX-Injected: 1"),
            Err(PathSegmentRejection::ControlCharacter)
        );
    }

    #[test]
    fn refuses_a_value_that_completes_a_dot_segment_with_its_literal_prefix() {
        assert_eq!(
            PathParameterValue::<DotPrefix>::new(".".to_string())
                .map(PathParameterValue::into_string),
            Err(PathSegmentRejection::DotSegment)
        );
    }

    #[test]
    fn keeps_a_value_that_only_forms_a_segment_after_its_literal_prefix() {
        assert_eq!(
            PathParameterValue::<ImagePrefix>::new(".".to_string())
                .map(PathParameterValue::into_string),
            Ok(".".to_string())
        );
    }

    #[test]
    fn refuses_an_empty_value_even_after_a_literal_prefix() {
        assert_eq!(
            PathParameterValue::<ImagePrefix>::new(String::new())
                .map(PathParameterValue::into_string),
            Err(PathSegmentRejection::Empty)
        );
    }
}
