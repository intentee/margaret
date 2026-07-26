use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;

use margaret_attributes::canonical_path::CanonicalPath;

use crate::dependency_kind::DependencyKind;
use crate::direct_construction::DirectConstruction;
use crate::provider::Provider;

fn own_fallible(construction: &DirectConstruction) -> bool {
    matches!(
        construction,
        DirectConstruction::Constructor { fallible: true, .. }
    )
}

fn resolve_provider(
    key: &CanonicalPath,
    providers: &BTreeMap<CanonicalPath, Provider>,
    resolved: &mut HashMap<CanonicalPath, bool>,
    visiting: &mut HashSet<CanonicalPath>,
) -> bool {
    if let Some(value) = resolved.get(key) {
        return *value;
    }

    if !visiting.insert(key.clone()) {
        return false;
    }

    let provider = &providers[key];
    let mut value = own_fallible(&provider.construction);

    for dependency in provider.dependencies() {
        if let DependencyKind::Single { provider_key } = dependency
            && resolve_provider(provider_key, providers, resolved, visiting)
        {
            value = true;
        }
    }

    visiting.remove(key);
    resolved.insert(key.clone(), value);

    value
}

pub(crate) struct AccessorFallibility {
    fallible: HashMap<CanonicalPath, bool>,
}

impl AccessorFallibility {
    pub(crate) fn from_plan(
        providers: &BTreeMap<CanonicalPath, Provider>,
        constructions: &BTreeMap<CanonicalPath, Provider>,
    ) -> Self {
        let mut fallible: HashMap<CanonicalPath, bool> = HashMap::new();
        let mut visiting: HashSet<CanonicalPath> = HashSet::new();

        for key in providers.keys() {
            resolve_provider(key, providers, &mut fallible, &mut visiting);
        }

        for (key, construction) in constructions {
            let mut value = own_fallible(&construction.construction);

            for dependency in construction.dependencies() {
                if let DependencyKind::Single { provider_key } = dependency
                    && resolve_provider(provider_key, providers, &mut fallible, &mut visiting)
                {
                    value = true;
                }
            }

            fallible.insert(key.clone(), value);
        }

        Self { fallible }
    }

    pub(crate) fn of(&self, key: &CanonicalPath) -> bool {
        self.fallible[key]
    }
}
