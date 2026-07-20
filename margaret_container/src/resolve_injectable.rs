use syn::Type;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_item::IndexedItem;

use crate::container_bindings::ContainerBindings;
use crate::injectable_resolution::InjectableResolution;
use crate::injected_dependency::InjectedDependency;
use crate::peel_target::peel_target;
use crate::raw_target::RawTarget;

fn resolved_single(
    provider_key: CanonicalPath,
    bindings: &ContainerBindings,
) -> Option<InjectedDependency> {
    let binding = bindings.provider(&provider_key)?;

    Some(if binding.is_interface {
        InjectedDependency::SingleInterface {
            field: binding.field_name.clone(),
            interface: provider_key,
        }
    } else {
        InjectedDependency::SingleConcrete {
            concrete: provider_key,
            field: binding.field_name.clone(),
        }
    })
}

#[must_use]
pub fn resolve_injectable(
    index: &AttributeIndex,
    item: &IndexedItem,
    declared: &Type,
    bindings: &ContainerBindings,
) -> InjectableResolution {
    let Some(target) = peel_target(declared) else {
        return InjectableResolution::UnsupportedShape;
    };

    match target {
        RawTarget::Single(written) => match index
            .resolve_item_path(item, &written)
            .and_then(|provider_key| resolved_single(provider_key, bindings))
        {
            Some(dependency) => InjectableResolution::Resolved(dependency),
            None => InjectableResolution::MissingProvider,
        },
        RawTarget::Collection(written) => match index
            .resolve_item_path(item, &written)
            .filter(|trait_path| index.is_indexed_trait(trait_path))
        {
            Some(trait_path) => InjectableResolution::Resolved(InjectedDependency::Collection {
                member_fields: bindings.collection_members(&trait_path).to_vec(),
                trait_path,
            }),
            None => InjectableResolution::MissingProvider,
        },
    }
}
