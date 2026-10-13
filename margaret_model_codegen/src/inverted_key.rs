use margaret_attributes::canonical_path::CanonicalPath;

#[derive(Eq, Hash, PartialEq)]
pub(crate) struct InvertedKey {
    pub(crate) key_field: String,
    pub(crate) related: CanonicalPath,
}
