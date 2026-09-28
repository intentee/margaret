use syn::Type;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_item::IndexedItem;

use crate::container_bindings::ContainerBindings;
use crate::framework_injection_role::FrameworkInjectionRole;
use crate::injectable_resolution::InjectableResolution;
use crate::injected_dependency::InjectedDependency;
use crate::parameter_target::ParameterTarget;
use crate::provider_binding::ProviderBinding;

fn resolved_single(
    provider_key: CanonicalPath,
    bindings: &ContainerBindings,
) -> InjectableResolution {
    match bindings.provider(&provider_key) {
        None => InjectableResolution::MissingProvider,
        Some(ProviderBinding {
            injection: FrameworkInjectionRole::Unmarked,
            field_name,
            ..
        }) => InjectableResolution::Resolved(InjectedDependency {
            concrete: provider_key,
            field: field_name.clone(),
        }),
        Some(ProviderBinding {
            injection:
                FrameworkInjectionRole::JwksClientStore(_) | FrameworkInjectionRole::JwksServerStore,
            ..
        }) => InjectableResolution::JwksSecretStoreByPath,
        Some(ProviderBinding {
            injection: FrameworkInjectionRole::OidcClient(_),
            ..
        }) => InjectableResolution::FrameworkOnly,
    }
}

#[must_use]
pub fn resolve_injectable(
    index: &AttributeIndex,
    item: &IndexedItem,
    declared: &Type,
    bindings: &ContainerBindings,
) -> InjectableResolution {
    match ParameterTarget::peel(index, item, declared) {
        ParameterTarget::Resolved { resolved, .. } => resolved_single(resolved, bindings),
        ParameterTarget::Unresolved { .. } => InjectableResolution::MissingProvider,
        ParameterTarget::UnsupportedShape => InjectableResolution::UnsupportedShape,
    }
}
