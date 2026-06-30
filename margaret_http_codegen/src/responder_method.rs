use quote::ToTokens;
use syn::Attribute;
use syn::FnArg;
use syn::Pat;
use syn::ReturnType;
use syn::Type;

use margaret_attributes::attribute_args::AttributeArgs;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::indexed_method::IndexedMethod;
use margaret_attributes::resolve_struct::resolve_struct;
use margaret_attributes::resolve_trait::resolve_trait;

use crate::http_codegen_error::HttpCodegenError;
use crate::registries::Registries;
use crate::responder_output::ResponderOutput;
use crate::responder_signature::ResponderSignature;
use crate::route_parameter::RouteParameter;
use crate::route_parameter_binding::RouteParameterBinding;
use crate::session_requirement::SessionRequirement;

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
            HttpCodegenError::MissingHttpRouteParameterBinder {
                responder: responder.to_string(),
                parameter: parameter.to_string(),
                written: written.clone(),
            }
        })?;
    let binder = registries.binders.get(&model).cloned().ok_or_else(|| {
        HttpCodegenError::MissingHttpRouteParameterBinder {
            responder: responder.to_string(),
            parameter: parameter.to_string(),
            written: written.clone(),
        }
    })?;

    let intent = match intent {
        Some(intent) => {
            if !registries.gates.contains_key(&model) {
                return Err(HttpCodegenError::MissingCrudGate {
                    responder: responder.to_string(),
                    parameter: parameter.to_string(),
                    written,
                });
            }

            Some(intent)
        }
        None => None,
    };

    Ok(RouteParameterBinding::Bound { binder, intent })
}

fn is_string(declared: &Type) -> bool {
    matches!(declared, Type::Path(type_path) if type_path.path.is_ident("String"))
}

fn is_authenticated_actor(declared: &Type) -> bool {
    matches!(
        declared,
        Type::Path(type_path)
            if type_path
                .path
                .segments
                .last()
                .is_some_and(|segment| segment.ident == "AuthenticatedActor")
    )
}

fn is_request(declared: &Type) -> bool {
    let Type::Reference(reference) = declared else {
        return false;
    };

    matches!(
        reference.elem.as_ref(),
        Type::Path(type_path)
            if type_path
                .path
                .segments
                .last()
                .is_some_and(|segment| segment.ident == "Request")
    )
}

fn session_requirement(declared: &Type) -> SessionRequirement {
    if is_authenticated_actor(declared) {
        SessionRequirement::Optional
    } else {
        SessionRequirement::Required
    }
}

fn find_marker<'attribute>(
    attributes: &'attribute [Attribute],
    selector: &AttributeSelector,
) -> Option<&'attribute Attribute> {
    attributes
        .iter()
        .find(|attribute| selector.matches(attribute.path()))
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

fn responder_output(
    method: &IndexedMethod,
    registries: &Registries,
    referencing_root: &str,
) -> ResponderOutput {
    let ReturnType::Type(_, declared) = &method.signature().output else {
        return ResponderOutput::Plain;
    };

    let Some(intercepted) = resolve_trait(declared, registries.trait_paths, referencing_root)
    else {
        return ResponderOutput::Plain;
    };

    match registries.interceptors.get(&intercepted) {
        Some(interceptor) => ResponderOutput::Intercepted {
            interceptor: interceptor.clone(),
        },
        None => ResponderOutput::Plain,
    }
}

pub(crate) fn responder_method(
    item: &IndexedItem,
    responder: &str,
    registries: &Registries,
) -> Result<ResponderSignature, HttpCodegenError> {
    let method =
        find_responder_method(item).ok_or_else(|| HttpCodegenError::MissingResponderMethod {
            responder: responder.to_string(),
        })?;
    let route_parameter_selector =
        AttributeSelector::parse("route_parameter").expect("a valid selector");
    let session_selector =
        AttributeSelector::parse("session_authenticated").expect("a valid selector");
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

        let route_parameter = find_marker(&pattern_type.attrs, &route_parameter_selector);
        let is_session = find_marker(&pattern_type.attrs, &session_selector).is_some();
        let is_current_request = is_request(&pattern_type.ty);

        if route_parameter.is_none() && !is_session && !is_current_request {
            return Err(HttpCodegenError::UnmarkedResponderParameter {
                responder: responder.to_string(),
                parameter: position.to_string(),
            });
        }

        let Pat::Ident(pattern_ident) = &*pattern_type.pat else {
            return Err(HttpCodegenError::RouteParameterNotIdentifier {
                responder: responder.to_string(),
                parameter: position.to_string(),
            });
        };

        let name = pattern_ident.ident.to_string();
        let binding = if let Some(attribute) = route_parameter {
            classify(
                responder,
                &name,
                &pattern_type.ty,
                attribute,
                registries,
                referencing_root,
            )?
        } else if is_session {
            RouteParameterBinding::SessionAuthenticated(session_requirement(&pattern_type.ty))
        } else {
            RouteParameterBinding::CurrentRequest
        };

        parameters.push(RouteParameter { name, binding });
    }

    Ok(ResponderSignature {
        output: responder_output(method, registries, referencing_root),
        parameters,
    })
}
