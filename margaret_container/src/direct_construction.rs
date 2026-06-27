use margaret_attributes::struct_shape::StructShape;

use crate::dependency_kind::DependencyKind;

pub(crate) enum DirectConstruction {
    Constructor {
        dependencies: Vec<DependencyKind>,
        is_async: bool,
        method: String,
    },
    Fieldless {
        shape: StructShape,
    },
}

impl DirectConstruction {
    pub(crate) fn dependencies(&self) -> &[DependencyKind] {
        match self {
            DirectConstruction::Constructor { dependencies, .. } => dependencies,
            DirectConstruction::Fieldless { .. } => &[],
        }
    }

    pub(crate) fn is_async(&self) -> bool {
        match self {
            DirectConstruction::Constructor { is_async, .. } => *is_async,
            DirectConstruction::Fieldless { .. } => false,
        }
    }
}
