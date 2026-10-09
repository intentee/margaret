use crate::path_segment_admission::PathSegmentAdmission;
use crate::path_segment_rejection::PathSegmentRejection;

const PATH_SEPARATOR: char = '/';

#[must_use]
pub fn admit_segment_text(text: &str) -> PathSegmentAdmission {
    if text.contains(PATH_SEPARATOR) {
        PathSegmentAdmission::Rejected(PathSegmentRejection::Separator)
    } else if text.chars().any(char::is_control) {
        PathSegmentAdmission::Rejected(PathSegmentRejection::ControlCharacter)
    } else {
        PathSegmentAdmission::Admitted
    }
}

#[cfg(test)]
mod tests {
    use super::admit_segment_text;
    use crate::path_segment_admission::PathSegmentAdmission;
    use crate::path_segment_rejection::PathSegmentRejection;

    #[test]
    fn rejects_text_that_contains_a_path_separator() {
        assert_eq!(
            admit_segment_text("rust/lang"),
            PathSegmentAdmission::Rejected(PathSegmentRejection::Separator)
        );
    }

    #[test]
    fn rejects_text_that_contains_a_control_character() {
        assert_eq!(
            admit_segment_text("a\r\nb"),
            PathSegmentAdmission::Rejected(PathSegmentRejection::ControlCharacter)
        );
    }

    #[test]
    fn admits_dots_and_reserved_characters_inside_a_segment() {
        assert_eq!(
            admit_segment_text(".. {x}?#%"),
            PathSegmentAdmission::Admitted
        );
    }
}
