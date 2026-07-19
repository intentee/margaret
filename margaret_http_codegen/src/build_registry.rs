use std::collections::HashMap;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_item::IndexedItem;

use crate::http_codegen_error::HttpCodegenError;

fn associated_model(
    index: &AttributeIndex,
    item: &IndexedItem,
    associated_type_name: &str,
) -> Option<CanonicalPath> {
    let associated_type = item
        .associated_types()
        .iter()
        .find(|associated_type| associated_type.name() == associated_type_name)?;
    let resolved = index.resolve_item_type(item, associated_type.ty())?;

    index.is_indexed_struct(&resolved).then_some(resolved)
}

pub(crate) fn build_registry(
    index: &AttributeIndex,
    marker: &str,
    associated_type_name: &str,
    missing: impl Fn(String) -> HttpCodegenError,
    ambiguous: impl Fn(String, String, String) -> HttpCodegenError,
) -> Result<HashMap<CanonicalPath, CanonicalPath>, HttpCodegenError> {
    let selector = AttributeSelector::from_marker(marker);
    let mut registry: HashMap<CanonicalPath, CanonicalPath> = HashMap::new();

    for item in index.items() {
        if !item
            .attributes()
            .iter()
            .any(|attribute| selector.matches(attribute.path()))
        {
            continue;
        }

        let provider = item.canonical_path().clone();
        let model = associated_model(index, item, associated_type_name)
            .ok_or_else(|| missing(provider.to_string()))?;

        if let Some(existing) = registry.get(&model) {
            return Err(ambiguous(
                model.to_string(),
                existing.to_string(),
                provider.to_string(),
            ));
        }

        registry.insert(model, provider);
    }

    Ok(registry)
}
