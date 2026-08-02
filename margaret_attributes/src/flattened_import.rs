use crate::canonical_path::CanonicalPath;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct FlattenedImport {
    pub(crate) name: String,
    pub(crate) path: CanonicalPath,
}
