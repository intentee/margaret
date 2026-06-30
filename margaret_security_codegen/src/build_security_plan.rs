use quote::quote;
use syn::Path;
use syn::PathSegment;
use syn::Token;
use syn::punctuated::Punctuated;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::resolve_struct::resolve_struct;

use crate::crud_gate::CrudGate;
use crate::security_codegen_error::SecurityCodegenError;
use crate::security_plan::SecurityPlan;
use crate::site_gate::SiteGate;

fn associated_struct(
    item: &IndexedItem,
    associated_type_name: &str,
    struct_paths: &[CanonicalPath],
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

    resolve_struct(associated_type.ty(), struct_paths, referencing_root)
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
    struct_paths: &[CanonicalPath],
) -> Result<CanonicalPath, SecurityCodegenError> {
    let item = index
        .items()
        .iter()
        .find(|item| item.canonical_path() == store)
        .expect("the store item is indexed");

    associated_struct(item, "Actor", struct_paths).ok_or_else(|| {
        SecurityCodegenError::StoreUserUnresolved {
            store: store.to_string(),
        }
    })
}

fn crud_gates(
    index: &AttributeIndex,
    actor_type: &CanonicalPath,
    struct_paths: &[CanonicalPath],
) -> Result<Vec<CrudGate>, SecurityCodegenError> {
    let selector = AttributeSelector::parse("decides_crud_action").expect("a valid selector");
    let mut gates: Vec<CrudGate> = Vec::new();

    for matched in index.select(&selector) {
        let item = matched.item();
        let gate_path = item.canonical_path().clone();
        let gate_user = associated_struct(item, "Actor", struct_paths).ok_or_else(|| {
            SecurityCodegenError::CrudGateUserUnresolved {
                gate: gate_path.to_string(),
            }
        })?;

        if &gate_user != actor_type {
            return Err(SecurityCodegenError::MismatchedCrudGateUser {
                gate: gate_path.to_string(),
                gate_user: gate_user.to_string(),
                store_user: actor_type.to_string(),
            });
        }

        let subject_type = associated_struct(item, "Subject", struct_paths).ok_or_else(|| {
            SecurityCodegenError::CrudGateSubjectUnresolved {
                gate: gate_path.to_string(),
            }
        })?;

        if let Some(existing) = gates.iter().find(|gate| gate.subject_type == subject_type) {
            return Err(SecurityCodegenError::AmbiguousCrudActionGate {
                subject: subject_type.to_string(),
                first: existing.gate_path.to_string(),
                second: gate_path.to_string(),
            });
        }

        gates.push(CrudGate {
            gate_path,
            subject_type,
        });
    }

    gates.sort_by(|first, second| first.gate_path.cmp(&second.gate_path));

    Ok(gates)
}

fn site_gates(
    index: &AttributeIndex,
    actor_type: &CanonicalPath,
    struct_paths: &[CanonicalPath],
) -> Result<Vec<SiteGate>, SecurityCodegenError> {
    let selector = AttributeSelector::parse("decides_site_action").expect("a valid selector");
    let mut gates: Vec<SiteGate> = Vec::new();
    let mut action_type: Option<Path> = None;

    for matched in index.select(&selector) {
        let item = matched.item();
        let gate_path = item.canonical_path().clone();
        let gate_user = associated_struct(item, "Actor", struct_paths).ok_or_else(|| {
            SecurityCodegenError::SiteActionGateUserUnresolved {
                gate: gate_path.to_string(),
            }
        })?;

        if &gate_user != actor_type {
            return Err(SecurityCodegenError::MismatchedSiteActionGateUser {
                gate: gate_path.to_string(),
                gate_user: gate_user.to_string(),
                store_user: actor_type.to_string(),
            });
        }

        let action_path = matched
            .args()?
            .positional_path(0)
            .ok_or_else(|| SecurityCodegenError::MissingSiteActionArgument {
                gate: gate_path.to_string(),
            })?
            .clone();
        let gate_action_type = enum_path(&action_path).ok_or_else(|| {
            SecurityCodegenError::MalformedSiteActionPath {
                gate: gate_path.to_string(),
                written: quote! { #action_path }.to_string(),
            }
        })?;

        match &action_type {
            Some(expected) if expected != &gate_action_type => {
                return Err(SecurityCodegenError::MismatchedSiteActionType {
                    gate: gate_path.to_string(),
                    action_type: quote! { #gate_action_type }.to_string(),
                    expected: quote! { #expected }.to_string(),
                });
            }
            Some(_) => {}
            None => action_type = Some(gate_action_type),
        }

        if let Some(existing) = gates.iter().find(|gate| gate.action_path == action_path) {
            return Err(SecurityCodegenError::AmbiguousSiteActionGate {
                action: quote! { #action_path }.to_string(),
                first: existing.gate_path.to_string(),
                second: gate_path.to_string(),
            });
        }

        gates.push(SiteGate {
            action_path,
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

    let struct_paths = index.struct_paths();
    let actor_type = store_user(index, &store_path, &struct_paths)?;
    let crud_gates = crud_gates(index, &actor_type, &struct_paths)?;
    let site_gates = site_gates(index, &actor_type, &struct_paths)?;
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
