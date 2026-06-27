use quote::ToTokens;
use syn::Attribute;
use syn::FnArg;
use syn::Pat;
use syn::Type;

use margaret_attributes::attribute_args::AttributeArgs;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::indexed_method::IndexedMethod;

use crate::authorization::Authorization;
use crate::http_codegen_error::HttpCodegenError;
use crate::registries::Registries;
use crate::resolve_struct::resolve_struct;
use crate::route_parameter::RouteParameter;
use crate::route_parameter_binding::RouteParameterBinding;

pub(crate) fn responder_method(
    item: &IndexedItem,
    responder: &str,
    registries: &Registries,
) -> Result<Vec<RouteParameter>, HttpCodegenError> {
    let method =
        find_responder_method(item).ok_or_else(|| HttpCodegenError::MissingResponderMethod {
            responder: responder.to_string(),
        })?;
    let route_parameter_selector =
        AttributeSelector::parse("route_parameter").expect("a valid selector");
    let referencing_root = item
        .canonical_path()
        .segments()
        .first()
        .expect("a canonical path has at least one segment");
    let mut parameters = Vec::new();

    for (position, input) in method.signature().inputs.iter().enumerate() {
        let FnArg::Typed(pattern_type) = input else {
            continue;
        };

        let Some(attribute) = pattern_type
            .attrs
            .iter()
            .find(|attribute| route_parameter_selector.matches(attribute.path()))
        else {
            return Err(HttpCodegenError::UnmarkedResponderParameter {
                responder: responder.to_string(),
                parameter: position.to_string(),
            });
        };

        let Pat::Ident(pattern_ident) = &*pattern_type.pat else {
            return Err(HttpCodegenError::RouteParameterNotIdentifier {
                responder: responder.to_string(),
                parameter: position.to_string(),
            });
        };

        let name = pattern_ident.ident.to_string();
        let binding = classify(
            responder,
            &name,
            &pattern_type.ty,
            attribute,
            registries,
            referencing_root,
        )?;

        parameters.push(RouteParameter { name, binding });
    }

    Ok(parameters)
}

fn classify(
    responder: &str,
    parameter: &str,
    declared: &Type,
    attribute: &Attribute,
    registries: &Registries,
    referencing_root: &str,
) -> Result<RouteParameterBinding, HttpCodegenError> {
    let intent = AttributeArgs::from_attribute(attribute)?.path("intent")?;

    if is_string(declared) {
        if intent.is_some() {
            return Err(HttpCodegenError::IntentOnRawParameter {
                responder: responder.to_string(),
                parameter: parameter.to_string(),
            });
        }

        return Ok(RouteParameterBinding::Raw);
    }

    let written = declared.to_token_stream().to_string();
    let model =
        resolve_struct(declared, registries.struct_paths, referencing_root).ok_or_else(|| {
            HttpCodegenError::MissingRouteParameterBinder {
                responder: responder.to_string(),
                parameter: parameter.to_string(),
                written: written.clone(),
            }
        })?;
    let binder = registries.binders.get(&model).cloned().ok_or_else(|| {
        HttpCodegenError::MissingRouteParameterBinder {
            responder: responder.to_string(),
            parameter: parameter.to_string(),
            written: written.clone(),
        }
    })?;

    let authorization = match intent {
        Some(intent) => {
            let gate = registries.gates.get(&model).cloned().ok_or_else(|| {
                HttpCodegenError::MissingCrudGate {
                    responder: responder.to_string(),
                    parameter: parameter.to_string(),
                    written,
                }
            })?;

            Some(Authorization { gate, intent })
        }
        None => None,
    };

    Ok(RouteParameterBinding::Bound {
        binder,
        authorization,
    })
}

fn is_string(declared: &Type) -> bool {
    matches!(declared, Type::Path(type_path) if type_path.path.is_ident("String"))
}

fn find_responder_method(item: &IndexedItem) -> Option<&IndexedMethod> {
    let responder_selector = AttributeSelector::parse("responder").expect("a valid selector");

    item.methods().iter().find(|method| {
        method
            .attributes()
            .iter()
            .any(|attribute| responder_selector.matches(attribute.path()))
    })
}
