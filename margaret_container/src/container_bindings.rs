use std::collections::BTreeMap;

use margaret_attributes::canonical_path::CanonicalPath;

use crate::container_plan::ContainerPlan;
use crate::provided_type::ProvidedType;
use crate::provider_binding::ProviderBinding;

pub struct ContainerBindings {
    collections: BTreeMap<CanonicalPath, Vec<String>>,
    providers: BTreeMap<CanonicalPath, ProviderBinding>,
}

impl ContainerBindings {
    pub(crate) fn from_plan(plan: &ContainerPlan) -> Self {
        let providers = plan
            .providers
            .iter()
            .map(|(provider_key, provider)| {
                (
                    provider_key.clone(),
                    ProviderBinding {
                        field_name: provider.field_name.clone(),
                        is_interface: matches!(provider.provided, ProvidedType::Interface(_)),
                    },
                )
            })
            .collect();
        let collections = plan
            .collections
            .entries()
            .map(|(trait_path, members)| {
                let member_fields = members
                    .iter()
                    .map(|member_key| plan.providers[member_key].field_name.clone())
                    .collect();

                (trait_path.clone(), member_fields)
            })
            .collect();

        Self {
            collections,
            providers,
        }
    }

    #[must_use]
    pub fn provides(&self, provider_key: &CanonicalPath) -> bool {
        self.providers.contains_key(provider_key)
    }

    pub(crate) fn collection_members(&self, trait_path: &CanonicalPath) -> &[String] {
        match self.collections.get(trait_path) {
            Some(member_fields) => member_fields,
            None => &[],
        }
    }

    pub(crate) fn provider(&self, provider_key: &CanonicalPath) -> Option<&ProviderBinding> {
        self.providers.get(provider_key)
    }
}
