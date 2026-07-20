use proc_macro2::TokenStream;

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
    Provided {
        expression: TokenStream,
    },
}

impl DirectConstruction {
    pub(crate) fn dependencies(&self) -> &[DependencyKind] {
        match self {
            DirectConstruction::Constructor { dependencies, .. } => dependencies,
            DirectConstruction::Fieldless { .. } | DirectConstruction::Provided { .. } => &[],
        }
    }
}
