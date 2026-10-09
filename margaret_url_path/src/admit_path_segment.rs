use crate::admit_segment_text::admit_segment_text;
use crate::path_segment_admission::PathSegmentAdmission;
use crate::path_segment_rejection::PathSegmentRejection;

const CURRENT_DIRECTORY_SEGMENT: &str = ".";
const PARENT_DIRECTORY_SEGMENT: &str = "..";

#[must_use]
pub fn admit_path_segment(segment: &str) -> PathSegmentAdmission {
    if segment.is_empty() {
        PathSegmentAdmission::Rejected(PathSegmentRejection::Empty)
    } else if segment == CURRENT_DIRECTORY_SEGMENT || segment == PARENT_DIRECTORY_SEGMENT {
        PathSegmentAdmission::Rejected(PathSegmentRejection::DotSegment)
    } else {
        admit_segment_text(segment)
    }
}

#[cfg(test)]
mod tests {
    use super::admit_path_segment;
    use crate::path_segment_admission::PathSegmentAdmission;
    use crate::path_segment_rejection::PathSegmentRejection;

    #[test]
    fn rejects_an_empty_segment() {
        assert_eq!(
            admit_path_segment(""),
            PathSegmentAdmission::Rejected(PathSegmentRejection::Empty)
        );
    }

    #[test]
    fn rejects_the_current_directory_segment() {
        assert_eq!(
            admit_path_segment("."),
            PathSegmentAdmission::Rejected(PathSegmentRejection::DotSegment)
        );
    }

    #[test]
    fn rejects_the_parent_directory_segment() {
        assert_eq!(
            admit_path_segment(".."),
            PathSegmentAdmission::Rejected(PathSegmentRejection::DotSegment)
        );
    }

    #[test]
    fn rejects_a_segment_whose_text_cannot_stay_inside_it() {
        assert_eq!(
            admit_path_segment("a/b"),
            PathSegmentAdmission::Rejected(PathSegmentRejection::Separator)
        );
    }

    #[test]
    fn admits_a_segment_that_merely_contains_dots() {
        assert_eq!(admit_path_segment("..."), PathSegmentAdmission::Admitted);
    }
}
