use std::collections::HashMap;

use margaret_attributes::canonical_path::CanonicalPath;

pub(crate) struct Registries<'registry> {
    pub(crate) binders: &'registry HashMap<CanonicalPath, CanonicalPath>,
    pub(crate) gates: &'registry HashMap<CanonicalPath, CanonicalPath>,
    pub(crate) struct_paths: &'registry [CanonicalPath],
}
