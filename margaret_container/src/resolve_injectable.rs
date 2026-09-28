use syn::Type;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_item::IndexedItem;

use crate::container_bindings::ContainerBindings;
use crate::injectable_resolution::InjectableResolution;
use crate::injected_dependency::InjectedDependency;
use crate::peel_target::peel_target;

fn resolved_single(
    provider_key: CanonicalPath,
    bindings: &ContainerBindings,
) -> Option<InjectedDependency> {
    let binding = bindings.provider(&provider_key)?;

    Some(InjectedDependency {
        concrete: provider_key,
        field: binding.field_name.clone(),
    })
}

#[must_use]
pub fn resolve_injectable(
    index: &AttributeIndex,
    item: &IndexedItem,
    declared: &Type,
    bindings: &ContainerBindings,
) -> InjectableResolution {
    let Some(written) = peel_target(index, item, declared) else {
        return InjectableResolution::UnsupportedShape;
    };

    match index
        .resolve_item_path(item, &written)
        .and_then(|provider_key| resolved_single(provider_key, bindings))
    {
        Some(dependency) => InjectableResolution::Resolved(dependency),
        None => InjectableResolution::MissingProvider,
    }
}
