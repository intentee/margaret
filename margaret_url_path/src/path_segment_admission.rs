use crate::path_segment_rejection::PathSegmentRejection;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PathSegmentAdmission {
    Admitted,
    Rejected(PathSegmentRejection),
}
