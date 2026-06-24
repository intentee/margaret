use margaret_attributes::canonical_path::CanonicalPath;

use crate::dependency_kind::DependencyKind;
use crate::provided_type::ProvidedType;

pub(crate) struct Provider {
    pub(crate) concrete_path: CanonicalPath,
    pub(crate) constructor_method: String,
    pub(crate) dependencies: Vec<DependencyKind>,
    pub(crate) field_name: String,
    pub(crate) provided: ProvidedType,
}
