use std::slice;

use margaret_attributes::struct_shape::StructShape;

use crate::constructor_outcome::ConstructorOutcome;
use crate::dependency_kind::DependencyKind;

#[derive(Clone)]
pub(crate) enum DirectConstruction {
    Constructor {
        dependencies: Vec<DependencyKind>,
        is_async: bool,
        method: String,
        outcome: ConstructorOutcome,
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
    FrameworkPrepared {
        dependency: DependencyKind,
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
            DirectConstruction::FrameworkPrepared { dependency } => slice::from_ref(dependency),
            DirectConstruction::Fieldless { .. } | DirectConstruction::FrameworkUnit => &[],
        }
    }

    pub(crate) fn outcome(&self) -> ConstructorOutcome {
        match self {
            Self::Constructor { outcome, .. } | Self::FrameworkConstructor { outcome, .. } => {
                *outcome
            }
            Self::Fieldless { .. }
            | Self::FrameworkPrepared { .. }
            | Self::FrameworkUnit
            | Self::FrameworkAccessor { .. } => ConstructorOutcome::Infallible,
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
