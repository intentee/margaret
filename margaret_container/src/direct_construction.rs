use margaret_attributes::canonical_path::CanonicalPath;
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
    Resolved {
        dependencies: Vec<DependencyKind>,
        resolver: CanonicalPath,
    },
}

impl DirectConstruction {
    pub(crate) fn dependencies(&self) -> &[DependencyKind] {
        match self {
            DirectConstruction::Constructor { dependencies, .. }
            | DirectConstruction::Resolved { dependencies, .. } => dependencies,
            DirectConstruction::Fieldless { .. } => &[],
        }
    }
}
