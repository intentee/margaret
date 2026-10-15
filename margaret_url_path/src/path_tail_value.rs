use std::marker::PhantomData;

use crate::admit_path_segment::admit_path_segment;
use crate::path_segment::PathSegment;
use crate::path_segment_admission::PathSegmentAdmission;
use crate::path_segment_rejection::PathSegmentRejection;
use crate::path_segments::path_segments;
use crate::segment_prefix::SegmentPrefix;

pub struct PathTailValue<TPrefix> {
    prefix: PhantomData<fn() -> TPrefix>,
    value: String,
}

impl<TPrefix: SegmentPrefix> PathTailValue<TPrefix> {
    /// # Errors
    ///
    /// Returns `PathSegmentRejection::Empty` for an empty tail, and the `PathSegmentRejection` of
    /// the first segment of the tail, joined to the literal text preceding it, that cannot fill a
    /// path segment.
    pub fn new(value: String) -> Result<Self, PathSegmentRejection> {
        if value.is_empty() {
            return Err(PathSegmentRejection::Empty);
        }

        let rejection =
            path_segments(&[TPrefix::TEXT, &value].concat()).find_map(|segment| match segment {
                PathSegment::Text(text) => match admit_path_segment(text) {
                    PathSegmentAdmission::Admitted => None,
                    PathSegmentAdmission::Rejected(rejection) => Some(rejection),
                },
                PathSegment::TrailingSeparator => None,
            });

        match rejection {
            Some(rejection) => Err(rejection),
            None => Ok(Self {
                prefix: PhantomData,
                value,
            }),
        }
    }

    #[must_use]
    pub fn into_string(self) -> String {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::PathTailValue;
    use crate::path_segment_rejection::PathSegmentRejection;
    use crate::segment_prefix::SegmentPrefix;
    use crate::unprefixed_segment::UnprefixedSegment;

    struct LetterPrefix;

    impl SegmentPrefix for LetterPrefix {
        const TEXT: &'static str = "x";
    }

    fn unprefixed(value: &str) -> Result<String, PathSegmentRejection> {
        PathTailValue::<UnprefixedSegment>::new(value.to_string()).map(PathTailValue::into_string)
    }

    #[test]
    fn keeps_path_separators_inside_a_tail() {
        assert_eq!(
            unprefixed("images/logo .png"),
            Ok("images/logo .png".to_string())
        );
    }

    #[test]
    fn keeps_a_trailing_separator_of_a_tail() {
        assert_eq!(unprefixed("images/"), Ok("images/".to_string()));
    }

    #[test]
    fn keeps_a_tail_that_opens_a_new_segment_after_its_literal_prefix() {
        assert_eq!(
            PathTailValue::<LetterPrefix>::new("/a b".to_string()).map(PathTailValue::into_string),
            Ok("/a b".to_string())
        );
    }

    #[test]
    fn refuses_an_empty_tail_since_it_cannot_address_the_route() {
        assert_eq!(unprefixed(""), Err(PathSegmentRejection::Empty));
    }

    #[test]
    fn refuses_a_tail_with_an_empty_component() {
        assert_eq!(unprefixed("a//b"), Err(PathSegmentRejection::Empty));
    }

    #[test]
    fn refuses_a_tail_with_a_dot_component() {
        assert_eq!(
            unprefixed("nested/../secret"),
            Err(PathSegmentRejection::DotSegment)
        );
    }
}
