use margaret_attributes::struct_shape::StructShape;

use crate::constructor_outcome::ConstructorOutcome;
use crate::dependency_kind::DependencyKind;

#[derive(Clone)]
pub(crate) enum DirectConstruction {
    Constructor {
        dependencies: Vec<DependencyKind>,
        is_async: bool,
        method: String,
    },
    Fieldless {
        shape: StructShape,
    },
    FrameworkConstructor {
        dependencies: Vec<DependencyKind>,
        is_async: bool,
        method: String,
        outcome: ConstructorOutcome,
    },
    FrameworkUnit,
    FrameworkAccessor {
        accessor: String,
        dependencies: Vec<DependencyKind>,
    },
}

impl DirectConstruction {
    pub(crate) fn dependencies(&self) -> &[DependencyKind] {
        match self {
            DirectConstruction::Constructor { dependencies, .. }
            | DirectConstruction::FrameworkConstructor { dependencies, .. }
            | DirectConstruction::FrameworkAccessor { dependencies, .. } => dependencies,
            DirectConstruction::Fieldless { .. } | DirectConstruction::FrameworkUnit => &[],
        }
    }

    pub(crate) fn is_async(&self) -> bool {
        matches!(
            self,
            Self::Constructor { is_async: true, .. }
                | Self::FrameworkConstructor { is_async: true, .. }
        )
    }
}
