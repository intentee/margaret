use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::tag::Tag;

pub struct SegmentedTagBinding {
    pub declaring: CanonicalPath,
    pub module_segment: String,
    pub tag: Tag,
}
