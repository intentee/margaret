use std::collections::HashMap;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::resolution_index::ResolutionIndex;
use margaret_attributes::resolve_trait::resolve_trait;

use crate::http_codegen_error::HttpCodegenError;

pub(crate) fn interceptor_bindings(
    index: &AttributeIndex,
    trait_resolution: &ResolutionIndex,
) -> Result<HashMap<CanonicalPath, CanonicalPath>, HttpCodegenError> {
    let selector = AttributeSelector::parse("intercepts").expect("a valid selector");
    let mut registry: HashMap<CanonicalPath, CanonicalPath> = HashMap::new();

    for item in index.items() {
        if !item
            .attributes()
            .iter()
            .any(|attribute| selector.matches(attribute.path()))
        {
            continue;
        }

        let interceptor = item.canonical_path().clone();
        let intercepted = item
            .associated_types()
            .iter()
            .find(|associated_type| associated_type.name() == "Intercepted")
            .and_then(|associated_type| resolve_trait(associated_type.ty(), trait_resolution))
            .ok_or_else(|| HttpCodegenError::MissingInterceptedType {
                interceptor: interceptor.to_string(),
            })?;

        if let Some(existing) = registry.get(&intercepted) {
            return Err(HttpCodegenError::AmbiguousInterceptor {
                intercepted: intercepted.to_string(),
                first: existing.to_string(),
                second: interceptor.to_string(),
            });
        }

        registry.insert(intercepted, interceptor);
    }

    Ok(registry)
}
