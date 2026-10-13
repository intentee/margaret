use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::tag::Tag;

pub(crate) struct MarkedStart {
    pub(crate) client: Tag,
    pub(crate) route: CanonicalPath,
}
