use std::collections::HashMap;

use syn::FnArg;
use syn::Type;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::indexed_method::IndexedMethod;
use margaret_attributes::resolution_index::ResolutionIndex;
use margaret_attributes::resolve_struct::resolve_struct;
use margaret_attributes::type_leaf_ident::type_leaf_ident;

use crate::http_codegen_error::HttpCodegenError;

fn decides_method(item: &IndexedItem) -> Option<&IndexedMethod> {
    item.method_matching(&AttributeSelector::parse("decides").expect("a valid selector"))
}

fn is_authenticated_actor_path(ty: &Type) -> bool {
    type_leaf_ident(ty).is_some_and(|ident| ident == "AuthenticatedActor")
}

fn gate_subject(
    method: &IndexedMethod,
    struct_resolution: &ResolutionIndex,
    referencing_root: &str,
) -> Option<CanonicalPath> {
    method.signature().inputs.iter().find_map(|input| {
        let FnArg::Typed(pattern_type) = input else {
            return None;
        };
        let Type::Reference(reference) = pattern_type.ty.as_ref() else {
            return None;
        };

        if is_authenticated_actor_path(&reference.elem) {
            return None;
        }

        resolve_struct(&reference.elem, struct_resolution, referencing_root)
    })
}

pub(crate) fn crud_gate_subjects(
    index: &AttributeIndex,
    struct_resolution: &ResolutionIndex,
) -> Result<HashMap<CanonicalPath, CanonicalPath>, HttpCodegenError> {
    let selector = AttributeSelector::parse("decides_crud_action").expect("a valid selector");
    let mut registry: HashMap<CanonicalPath, CanonicalPath> = HashMap::new();

    for item in index.items() {
        if !item
            .attributes()
            .iter()
            .any(|attribute| selector.matches(attribute.path()))
        {
            continue;
        }

        let gate = item.canonical_path().clone();
        let referencing_root = gate
            .segments()
            .first()
            .expect("a canonical path has at least one segment");
        let subject = decides_method(item)
            .and_then(|method| gate_subject(method, struct_resolution, referencing_root))
            .ok_or_else(|| HttpCodegenError::CrudGateSubject {
                gate: gate.to_string(),
            })?;

        if let Some(existing) = registry.get(&subject) {
            return Err(HttpCodegenError::AmbiguousCrudGate {
                subject: subject.to_string(),
                first: existing.to_string(),
                second: gate.to_string(),
            });
        }

        registry.insert(subject, gate);
    }

    Ok(registry)
}
