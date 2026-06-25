use margaret_attributes::struct_shape::StructShape;

use crate::dependency_kind::DependencyKind;

pub(crate) enum ProviderConstruction {
    Constructor {
        method: String,
        dependencies: Vec<DependencyKind>,
    },
    Fieldless {
        shape: StructShape,
    },
}
