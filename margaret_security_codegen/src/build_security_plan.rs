use std::collections::HashMap;

use quote::quote;
use syn::FnArg;
use syn::GenericArgument;
use syn::Path;
use syn::PathArguments;
use syn::PathSegment;
use syn::Token;
use syn::Type;
use syn::punctuated::Punctuated;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::indexed_method::IndexedMethod;
use margaret_attributes::resolution_index::ResolutionIndex;
use margaret_attributes::resolve_struct::resolve_struct;
use margaret_attributes::type_leaf_ident::type_leaf_ident;

use crate::actor_requirement::ActorRequirement;
use crate::crud_gate::CrudGate;
use crate::security_codegen_error::SecurityCodegenError;
use crate::security_plan::SecurityPlan;
use crate::site_action_arguments::SiteActionArguments;
use crate::site_gate::SiteGate;

fn decides_method(item: &IndexedItem) -> Option<&IndexedMethod> {
    item.method_matching(&AttributeSelector::parse("decides").expect("a valid selector"))
}

fn is_authenticated_actor_path(ty: &Type) -> bool {
    type_leaf_ident(ty).is_some_and(|ident| ident == "AuthenticatedActor")
}

fn is_authenticated_actor_reference(ty: &Type) -> bool {
    matches!(ty, Type::Reference(reference) if is_authenticated_actor_path(&reference.elem))
}

fn is_optional_authenticated_actor_reference(ty: &Type) -> bool {
    let Type::Path(type_path) = ty else {
        return false;
    };

    type_path.path.segments.last().is_some_and(|segment| {
        segment.ident == "Option"
            && matches!(
                &segment.arguments,
                PathArguments::AngleBracketed(arguments)
                    if matches!(
                        arguments.args.first(),
                        Some(GenericArgument::Type(inner)) if is_authenticated_actor_reference(inner)
                    )
            )
    })
}

fn actor_requirement(method: &IndexedMethod) -> Option<ActorRequirement> {
    method.signature().inputs.iter().find_map(|input| {
        let FnArg::Typed(pattern_type) = input else {
            return None;
        };

        if is_authenticated_actor_reference(&pattern_type.ty) {
            return Some(ActorRequirement::Required);
        }

        if is_optional_authenticated_actor_reference(&pattern_type.ty) {
            return Some(ActorRequirement::Optional);
        }

        None
    })
}

fn subject_type(
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

fn associated_struct(
    item: &IndexedItem,
    associated_type_name: &str,
    struct_resolution: &ResolutionIndex,
) -> Option<CanonicalPath> {
    let referencing_root = item
        .canonical_path()
        .segments()
        .first()
        .expect("a canonical path has at least one segment");
    let associated_type = item
        .associated_types()
        .iter()
        .find(|associated_type| associated_type.name() == associated_type_name)?;

    resolve_struct(associated_type.ty(), struct_resolution, referencing_root)
}

fn enum_path(action_path: &Path) -> Option<Path> {
    if action_path.segments.len() < 2 {
        return None;
    }

    let mut segments: Punctuated<PathSegment, Token![::]> = Punctuated::new();

    for segment in action_path
        .segments
        .iter()
        .take(action_path.segments.len() - 1)
    {
        segments.push(segment.clone());
    }

    Some(Path {
        leading_colon: action_path.leading_colon,
        segments,
    })
}

fn store_path(index: &AttributeIndex) -> Result<Option<CanonicalPath>, SecurityCodegenError> {
    let selector =
        AttributeSelector::parse("provides_authenticated_actor").expect("a valid selector");
    let mut store: Option<CanonicalPath> = None;

    for matched in index.select(&selector) {
        let path = matched.item().canonical_path().clone();

        if let Some(existing) = &store {
            return Err(SecurityCodegenError::AmbiguousAuthenticatedActorStore {
                first: existing.to_string(),
                second: path.to_string(),
            });
        }

        store = Some(path);
    }

    Ok(store)
}

fn store_user(
    index: &AttributeIndex,
    store: &CanonicalPath,
    struct_resolution: &ResolutionIndex,
) -> Result<CanonicalPath, SecurityCodegenError> {
    let item = index
        .items()
        .iter()
        .find(|item| item.canonical_path() == store)
        .expect("the store item is indexed");

    associated_struct(item, "Actor", struct_resolution).ok_or_else(|| {
        SecurityCodegenError::StoreUserUnresolved {
            store: store.to_string(),
        }
    })
}

fn crud_gates(
    index: &AttributeIndex,
    struct_resolution: &ResolutionIndex,
) -> Result<Vec<CrudGate>, SecurityCodegenError> {
    let selector = AttributeSelector::parse("decides_crud_action").expect("a valid selector");
    let mut gates: Vec<CrudGate> = Vec::new();
    let mut subjects: HashMap<CanonicalPath, CanonicalPath> = HashMap::new();

    for matched in index.select(&selector) {
        let item = matched.item();
        let gate_path = item.canonical_path().clone();
        let referencing_root = item
            .canonical_path()
            .segments()
            .first()
            .expect("a canonical path has at least one segment");
        let method =
            decides_method(item).ok_or_else(|| SecurityCodegenError::CrudGateDecisionMissing {
                gate: gate_path.to_string(),
            })?;
        let requirement = actor_requirement(method).ok_or_else(|| {
            SecurityCodegenError::CrudGateActorMissing {
                gate: gate_path.to_string(),
            }
        })?;
        let subject =
            subject_type(method, struct_resolution, referencing_root).ok_or_else(|| {
                SecurityCodegenError::CrudGateSubjectMissing {
                    gate: gate_path.to_string(),
                }
            })?;

        if let Some(first) = subjects.get(&subject) {
            return Err(SecurityCodegenError::AmbiguousCrudActionGate {
                subject: subject.to_string(),
                first: first.to_string(),
                second: gate_path.to_string(),
            });
        }

        subjects.insert(subject.clone(), gate_path.clone());
        gates.push(CrudGate {
            actor_requirement: requirement,
            gate_path,
            subject_type: subject,
        });
    }

    gates.sort_by(|first, second| first.gate_path.cmp(&second.gate_path));

    Ok(gates)
}

fn site_gates(index: &AttributeIndex) -> Result<Vec<SiteGate>, SecurityCodegenError> {
    let selector = AttributeSelector::parse("decides_site_action").expect("a valid selector");
    let mut gates: Vec<SiteGate> = Vec::new();
    let mut actions: HashMap<Path, CanonicalPath> = HashMap::new();

    for matched in index.select(&selector) {
        let item = matched.item();
        let gate_path = item.canonical_path().clone();
        let method = decides_method(item).ok_or_else(|| {
            SecurityCodegenError::SiteActionGateDecisionMissing {
                gate: gate_path.to_string(),
            }
        })?;
        let requirement = actor_requirement(method).ok_or_else(|| {
            SecurityCodegenError::SiteActionGateActorMissing {
                gate: gate_path.to_string(),
            }
        })?;

        let SiteActionArguments {
            action: action_path,
        } = SiteActionArguments::parse(&matched.args()?, &gate_path.to_string())?;
        let gate_action_type = enum_path(&action_path).ok_or_else(|| {
            SecurityCodegenError::MalformedSiteActionPath {
                gate: gate_path.to_string(),
                written: quote! { #action_path }.to_string(),
            }
        })?;

        if let Some(first_gate) = gates.first() {
            let expected =
                enum_path(&first_gate.action_path).expect("a validated site action has an enum");

            if expected != gate_action_type {
                return Err(SecurityCodegenError::MismatchedSiteActionType {
                    gate: gate_path.to_string(),
                    action_type: quote! { #gate_action_type }.to_string(),
                    expected: quote! { #expected }.to_string(),
                });
            }
        }

        if let Some(first) = actions.get(&action_path) {
            return Err(SecurityCodegenError::AmbiguousSiteActionGate {
                action: quote! { #action_path }.to_string(),
                first: first.to_string(),
                second: gate_path.to_string(),
            });
        }

        actions.insert(action_path.clone(), gate_path.clone());
        gates.push(SiteGate {
            action_path,
            actor_requirement: requirement,
            gate_path,
        });
    }

    gates.sort_by(|first, second| first.gate_path.cmp(&second.gate_path));

    Ok(gates)
}

pub(crate) fn build_security_plan(
    index: &AttributeIndex,
) -> Result<Option<SecurityPlan>, SecurityCodegenError> {
    let Some(store_path) = store_path(index)? else {
        return Ok(None);
    };

    let struct_resolution = index.struct_resolution();
    let actor_type = store_user(index, &store_path, struct_resolution)?;
    let crud_gates = crud_gates(index, struct_resolution)?;
    let site_gates = site_gates(index)?;
    let action_type = site_gates
        .first()
        .map(|gate| enum_path(&gate.action_path).expect("a validated site action has an enum"));

    Ok(Some(SecurityPlan {
        action_type,
        crud_gates,
        site_gates,
        store_path,
        actor_type,
    }))
}
