use crate::segment_prefix::SegmentPrefix;

pub struct UnprefixedSegment;

impl SegmentPrefix for UnprefixedSegment {
    const TEXT: &'static str = "";
}
