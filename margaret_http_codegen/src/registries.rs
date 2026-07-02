use std::collections::HashMap;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::resolution_index::ResolutionIndex;

pub(crate) struct Registries<'registry> {
    pub(crate) binders: &'registry HashMap<CanonicalPath, CanonicalPath>,
    pub(crate) interceptors: &'registry HashMap<CanonicalPath, CanonicalPath>,
    pub(crate) struct_resolution: &'registry ResolutionIndex,
    pub(crate) trait_resolution: &'registry ResolutionIndex,
}
