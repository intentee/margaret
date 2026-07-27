use margaret_attributes::canonical_path::CanonicalPath;

use crate::dependency_kind::DependencyKind;
use crate::direct_construction::DirectConstruction;
use crate::provided_type::ProvidedType;

#[derive(Clone)]
pub(crate) struct Provider {
    pub(crate) concrete_path: CanonicalPath,
    pub(crate) construction: DirectConstruction,
    pub(crate) field_name: String,
    pub(crate) provided: ProvidedType,
    pub(crate) type_name: String,
}

impl Provider {
    pub(crate) fn dependencies(&self) -> &[DependencyKind] {
        self.construction.dependencies()
    }
}
