use margaret_url_path::path_segment_rejection::PathSegmentRejection;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnaddressableParameter {
    pub name: &'static str,
    pub rejection: PathSegmentRejection,
}
