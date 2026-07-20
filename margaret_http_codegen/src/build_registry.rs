use std::collections::HashMap;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_item::IndexedItem;

use crate::http_codegen_error::HttpCodegenError;
use crate::route_parameter_binder::RouteParameterBinder;

fn associated_model(
    index: &AttributeIndex,
    item: &IndexedItem,
    associated_type_name: &str,
) -> Option<CanonicalPath> {
    item.trait_impls().iter().find_map(|trait_impl| {
        let associated_type = trait_impl.associated_type(associated_type_name)?;
        let resolved = index.resolve_module_type(trait_impl.module_path(), associated_type.ty())?;

        index
            .struct_identifier(&resolved)
            .is_some()
            .then_some(resolved)
    })
}

pub(crate) fn build_registry(
    index: &AttributeIndex,
    marker: &str,
    associated_type_name: &str,
    not_a_struct: impl Fn(String) -> HttpCodegenError,
    missing: impl Fn(String) -> HttpCodegenError,
    ambiguous: impl Fn(String, String, String) -> HttpCodegenError,
) -> Result<HashMap<CanonicalPath, RouteParameterBinder>, HttpCodegenError> {
    let selector = AttributeSelector::from_marker(marker);
    let mut registry: HashMap<CanonicalPath, RouteParameterBinder> = HashMap::new();

    for item in index.items() {
        if !item
            .attributes()
            .iter()
            .any(|attribute| selector.matches(attribute.path()))
        {
            continue;
        }

        let provider = item.canonical_path().clone();
        let Some(identifier) = index.struct_identifier(&provider) else {
            return Err(not_a_struct(provider.to_string()));
        };
        let field = identifier.field().to_string();
        let model = associated_model(index, item, associated_type_name)
            .ok_or_else(|| missing(provider.to_string()))?;

        if let Some(existing) = registry.get(&model) {
            return Err(ambiguous(
                model.to_string(),
                existing.provider.to_string(),
                provider.to_string(),
            ));
        }

        registry.insert(model, RouteParameterBinder { field, provider });
    }

    Ok(registry)
}
