use crate::dependency_kind::DependencyKind;
use crate::direct_construction::DirectConstruction;

pub(crate) enum ProviderConstruction {
    Direct(DirectConstruction),
    Factory {
        factory_is_async: bool,
        factory_method: String,
        provider: DirectConstruction,
    },
}

impl ProviderConstruction {
    pub(crate) fn dependencies(&self) -> &[DependencyKind] {
        match self {
            ProviderConstruction::Direct(construction) => construction.dependencies(),
            ProviderConstruction::Factory { provider, .. } => provider.dependencies(),
        }
    }
}
