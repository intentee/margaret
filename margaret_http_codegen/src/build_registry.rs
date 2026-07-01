use std::collections::HashMap;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::resolution_index::ResolutionIndex;
use margaret_attributes::resolve_struct::resolve_struct;

use crate::http_codegen_error::HttpCodegenError;

fn associated_model(
    item: &IndexedItem,
    associated_type_name: &str,
    struct_resolution: &ResolutionIndex,
    referencing_root: &str,
) -> Option<CanonicalPath> {
    let associated_type = item
        .associated_types()
        .iter()
        .find(|associated_type| associated_type.name() == associated_type_name)?;

    resolve_struct(associated_type.ty(), struct_resolution, referencing_root)
}

pub(crate) fn build_registry(
    index: &AttributeIndex,
    struct_resolution: &ResolutionIndex,
    marker: &str,
    associated_type_name: &str,
    missing: impl Fn(String) -> HttpCodegenError,
    ambiguous: impl Fn(String, String, String) -> HttpCodegenError,
) -> Result<HashMap<CanonicalPath, CanonicalPath>, HttpCodegenError> {
    let selector = AttributeSelector::parse(marker).expect("a valid selector");
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
        let referencing_root = provider
            .segments()
            .first()
            .expect("a canonical path has at least one segment");
        let model = associated_model(
            item,
            associated_type_name,
            struct_resolution,
            referencing_root,
        )
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
