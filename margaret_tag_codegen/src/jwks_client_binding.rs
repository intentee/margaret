use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::tag::Tag;

pub struct JwksClientBinding {
    pub endpoint: CanonicalPath,
    pub tag: Tag,
}
