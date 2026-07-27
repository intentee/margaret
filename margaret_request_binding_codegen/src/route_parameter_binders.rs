use std::collections::HashMap;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_container::is_singleton::is_singleton;

use crate::request_binding_error::RequestBindingError;
use crate::route_parameter_binder::RouteParameterBinder;

fn associated_model(index: &AttributeIndex, item: &IndexedItem) -> Option<CanonicalPath> {
    item.trait_impls().iter().find_map(|trait_impl| {
        let associated_type = trait_impl.associated_type("Model")?;
        let resolved = index.resolve_module_type(trait_impl.module_path(), associated_type.ty())?;

        index
            .struct_identifier(&resolved)
            .is_some()
            .then_some(resolved)
    })
}

pub fn route_parameter_binders(
    index: &AttributeIndex,
) -> Result<HashMap<CanonicalPath, RouteParameterBinder>, RequestBindingError> {
    let mut registry: HashMap<CanonicalPath, RouteParameterBinder> = HashMap::new();

    for item in index.items() {
        if !item.has_framework_attribute(FrameworkAttribute::ProvidesRouteParameter) {
            continue;
        }

        let provider = item.canonical_path().clone();
        let Some(identifier) = index.struct_identifier(&provider) else {
            return Err(RequestBindingError::RouteParameterBinderNotAStruct {
                binder: provider.to_string(),
            });
        };

        if !is_singleton(item) {
            return Err(RequestBindingError::RouteParameterBinderRequiresSingleton {
                binder: provider.to_string(),
            });
        }

        let field = identifier.field().to_string();
        let model = associated_model(index, item).ok_or_else(|| {
            RequestBindingError::RouteParameterBinderModel {
                binder: provider.to_string(),
            }
        })?;

        if let Some(existing) = registry.get(&model) {
            return Err(RequestBindingError::AmbiguousRouteParameterBinder {
                model: model.to_string(),
                first: existing.provider.to_string(),
                second: provider.to_string(),
            });
        }

        registry.insert(model, RouteParameterBinder { field, provider });
    }

    Ok(registry)
}
